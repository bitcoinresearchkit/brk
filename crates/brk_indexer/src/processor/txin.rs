use brk_cohort::ByAddrType;
use brk_error::{Error, Result};
use brk_store::Store;
use brk_types::{
    AddrIndexOutPoint, AddrIndexTxIndex, OutPoint, OutputType, TxInIndex, TxIndex, Txid,
    TxidPrefix, TypeIndex, Unit, Vin, Vout,
};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::error;
use vecdb::{PcoVec, WritableVec};

use super::{BlockProcessor, ComputedTx, InputSource, SameBlockOutputInfo};
use crate::InputsVecs;

impl<'a> BlockProcessor<'a> {
    pub fn process_inputs(
        &self,
        txs: &[ComputedTx],
        txid_prefix_to_tx_index: &mut FxHashMap<TxidPrefix, TxIndex>,
    ) -> Result<Vec<(TxInIndex, InputSource)>> {
        txid_prefix_to_tx_index.clear();
        txid_prefix_to_tx_index.extend(txs.iter().map(|ct| (ct.txid_prefix, ct.tx_index)));

        let base_tx_index = self.indexes.tx_index;
        let base_txin_index = self.indexes.txin_index;

        let total_inputs: usize = self.block.txdata.iter().map(|tx| tx.input.len()).sum();
        let mut items = Vec::with_capacity(total_inputs);
        for (index, tx) in self.block.txdata.iter().enumerate() {
            for (vin, txin) in tx.input.iter().enumerate() {
                items.push((TxIndex::from(index), Vin::from(vin), txin, tx));
            }
        }

        let txid_prefix_to_tx_index = &*txid_prefix_to_tx_index;

        let txins = items
            .into_par_iter()
            .enumerate()
            .map(
                |(block_txin_index, (block_tx_index, vin, txin, tx))| -> Result<(TxInIndex, InputSource)> {
                    let tx_index = base_tx_index + block_tx_index;
                    let txin_index = base_txin_index + TxInIndex::from(block_txin_index);

                    if tx.is_coinbase() {
                        return Ok((
                            txin_index,
                            InputSource::SameBlock {
                                tx_index,
                                vin,
                                outpoint: OutPoint::COINBASE,
                            },
                        ));
                    }

                    let outpoint = txin.previous_output;
                    let txid = Txid::from(outpoint.txid);
                    let txid_prefix = TxidPrefix::from(&txid);
                    let vout = Vout::from(outpoint.vout);

                    if let Some(&same_block_tx_index) = txid_prefix_to_tx_index
                        .get(&txid_prefix) {
                        let outpoint = OutPoint::new(same_block_tx_index, vout);
                        return Ok((
                            txin_index,
                            InputSource::SameBlock {
                                tx_index,
                                vin,
                                outpoint,
                            },
                        ));
                    }

                    let store_result = self
                        .stores
                        .txid_prefix_to_tx_index
                        .get(&txid_prefix)?
                        .map(|v| *v);

                    let prev_tx_index = match store_result {
                        Some(tx_index) if tx_index < self.indexes.tx_index => tx_index,
                        _ => {
                            error!(
                                "UnknownTxid: txid={}, prefix={:?}, store_result={:?}, current_tx_index={:?}",
                                txid, txid_prefix, store_result, self.indexes.tx_index
                            );
                            return Err(Error::UnknownTxid);
                        }
                    };

                    let txout_index = self
                        .vecs
                        .transactions
                        .first_txout_index
                        .get_pushed_or_read(prev_tx_index, &self.readers.tx_index_to_first_txout_index)
                        .ok_or(Error::Internal("Missing txout_index"))?
                        + vout;

                    let outpoint = OutPoint::new(prev_tx_index, vout);

                    let output_type = self
                        .vecs
                        .outputs
                        .output_type
                        .get_pushed_or_read(txout_index, &self.readers.txout_index_to_output_type)
                        .ok_or(Error::Internal("Missing output_type"))?;

                    let type_index = self
                        .vecs
                        .outputs
                        .type_index
                        .get_pushed_or_read(txout_index, &self.readers.txout_index_to_type_index)
                        .ok_or(Error::Internal("Missing type_index"))?;

                    Ok((
                        txin_index,
                        InputSource::PreviousBlock {
                            vin,
                            tx_index,
                            outpoint,
                            output_type,
                            type_index,
                        },
                    ))
                },
            )
            .collect::<Result<Vec<_>>>()?;

        Ok(txins)
    }

    pub fn collect_same_block_spent_outpoints(
        txins: &[(TxInIndex, InputSource)],
        out: &mut FxHashSet<OutPoint>,
    ) {
        out.clear();
        out.extend(
            txins
                .iter()
                .filter_map(|(_, input_source)| match input_source {
                    InputSource::SameBlock { outpoint, .. } if !outpoint.is_coinbase() => {
                        Some(*outpoint)
                    }
                    _ => None,
                }),
        );
    }
}

pub(super) fn finalize_inputs(
    first_txin_index: &mut PcoVec<TxIndex, TxInIndex>,
    inputs: &mut InputsVecs,
    addr_tx_index_stores: &mut ByAddrType<Store<AddrIndexTxIndex, Unit>>,
    addr_outpoint_stores: &mut ByAddrType<Store<AddrIndexOutPoint, Unit>>,
    txins: Vec<(TxInIndex, InputSource)>,
    same_block_output_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
) -> Result<()> {
    for (txin_index, input_source) in txins {
        let (vin, tx_index, outpoint, output_type, type_index) = match input_source {
            InputSource::PreviousBlock {
                vin,
                tx_index,
                outpoint,
                output_type,
                type_index,
            } => (vin, tx_index, outpoint, output_type, type_index),
            InputSource::SameBlock {
                tx_index,
                vin,
                outpoint,
            } => {
                if outpoint.is_coinbase() {
                    (
                        vin,
                        tx_index,
                        outpoint,
                        OutputType::Unknown,
                        TypeIndex::COINBASE,
                    )
                } else {
                    let info = same_block_output_info
                        .remove(&outpoint)
                        .ok_or(Error::Internal("Same-block output not found"))
                        .inspect_err(|_| {
                            error!(
                                ?outpoint,
                                remaining = same_block_output_info.len(),
                                "Same-block output not found"
                            );
                        })?;
                    (vin, tx_index, outpoint, info.output_type, info.type_index)
                }
            }
        };

        if vin.is_zero() {
            first_txin_index.checked_push(tx_index, txin_index)?;
        }

        inputs.tx_index.checked_push(txin_index, tx_index)?;
        inputs.outpoint.checked_push(txin_index, outpoint)?;
        inputs.output_type.checked_push(txin_index, output_type)?;
        inputs.type_index.checked_push(txin_index, type_index)?;

        if !output_type.is_addr() {
            continue;
        }
        let addr_type = output_type;
        let addr_index = type_index;

        addr_tx_index_stores
            .get_mut_unwrap(addr_type)
            .insert(AddrIndexTxIndex::from((addr_index, tx_index)), Unit);

        addr_outpoint_stores
            .get_mut_unwrap(addr_type)
            .remove(AddrIndexOutPoint::from((addr_index, outpoint)));
    }

    Ok(())
}
