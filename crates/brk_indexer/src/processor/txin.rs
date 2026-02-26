use brk_cohort::ByAddressType;
use brk_error::{Error, Result};
use brk_store::Store;
use brk_types::{
    AddressIndexOutPoint, AddressIndexTxIndex, OutPoint, OutputType, TxInIndex, TxIndex, Txid,
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
        txid_prefix_to_txindex: &mut FxHashMap<TxidPrefix, TxIndex>,
    ) -> Result<Vec<(TxInIndex, InputSource)>> {
        txid_prefix_to_txindex.clear();
        txid_prefix_to_txindex.extend(txs.iter().map(|ct| (ct.txid_prefix, ct.txindex)));

        let base_txindex = self.indexes.txindex;
        let base_txinindex = self.indexes.txinindex;

        let total_inputs: usize = self.block.txdata.iter().map(|tx| tx.input.len()).sum();
        let mut items = Vec::with_capacity(total_inputs);
        for (index, tx) in self.block.txdata.iter().enumerate() {
            for (vin, txin) in tx.input.iter().enumerate() {
                items.push((TxIndex::from(index), Vin::from(vin), txin, tx));
            }
        }

        let txid_prefix_to_txindex = &*txid_prefix_to_txindex;

        let txins = items
            .into_par_iter()
            .enumerate()
            .map(
                |(block_txinindex, (block_txindex, vin, txin, tx))| -> Result<(TxInIndex, InputSource)> {
                    let txindex = base_txindex + block_txindex;
                    let txinindex = base_txinindex + TxInIndex::from(block_txinindex);

                    if tx.is_coinbase() {
                        return Ok((
                            txinindex,
                            InputSource::SameBlock {
                                txindex,
                                vin,
                                outpoint: OutPoint::COINBASE,
                            },
                        ));
                    }

                    let outpoint = txin.previous_output;
                    let txid = Txid::from(outpoint.txid);
                    let txid_prefix = TxidPrefix::from(&txid);
                    let vout = Vout::from(outpoint.vout);

                    if let Some(&same_block_txindex) = txid_prefix_to_txindex
                        .get(&txid_prefix) {
                        let outpoint = OutPoint::new(same_block_txindex, vout);
                        return Ok((
                            txinindex,
                            InputSource::SameBlock {
                                txindex,
                                vin,
                                outpoint,
                            },
                        ));
                    }

                    let store_result = self
                        .stores
                        .txidprefix_to_txindex
                        .get(&txid_prefix)?
                        .map(|v| *v);

                    let prev_txindex = match store_result {
                        Some(txindex) if txindex < self.indexes.txindex => txindex,
                        _ => {
                            error!(
                                "UnknownTxid: txid={}, prefix={:?}, store_result={:?}, current_txindex={:?}",
                                txid, txid_prefix, store_result, self.indexes.txindex
                            );
                            return Err(Error::UnknownTxid);
                        }
                    };

                    let txoutindex = self
                        .vecs
                        .transactions
                        .first_txoutindex
                        .get_pushed_or_read(prev_txindex, &self.readers.txindex_to_first_txoutindex)
                        .ok_or(Error::Internal("Missing txoutindex"))?
                        + vout;

                    let outpoint = OutPoint::new(prev_txindex, vout);

                    let outputtype = self
                        .vecs
                        .outputs
                        .outputtype
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_outputtype)
                        .ok_or(Error::Internal("Missing outputtype"))?;

                    let typeindex = self
                        .vecs
                        .outputs
                        .typeindex
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_typeindex)
                        .ok_or(Error::Internal("Missing typeindex"))?;

                    Ok((
                        txinindex,
                        InputSource::PreviousBlock {
                            vin,
                            txindex,
                            outpoint,
                            outputtype,
                            typeindex,
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
    first_txinindex: &mut PcoVec<TxIndex, TxInIndex>,
    inputs: &mut InputsVecs,
    addr_txindex_stores: &mut ByAddressType<Store<AddressIndexTxIndex, Unit>>,
    addr_outpoint_stores: &mut ByAddressType<Store<AddressIndexOutPoint, Unit>>,
    txins: Vec<(TxInIndex, InputSource)>,
    same_block_output_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
) -> Result<()> {
    for (txinindex, input_source) in txins {
        let (vin, txindex, outpoint, outputtype, typeindex) = match input_source {
            InputSource::PreviousBlock {
                vin,
                txindex,
                outpoint,
                outputtype,
                typeindex,
            } => (vin, txindex, outpoint, outputtype, typeindex),
            InputSource::SameBlock {
                txindex,
                vin,
                outpoint,
            } => {
                if outpoint.is_coinbase() {
                    (
                        vin,
                        txindex,
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
                    (vin, txindex, outpoint, info.outputtype, info.typeindex)
                }
            }
        };

        if vin.is_zero() {
            first_txinindex.checked_push(txindex, txinindex)?;
        }

        inputs.txindex.checked_push(txinindex, txindex)?;
        inputs.outpoint.checked_push(txinindex, outpoint)?;
        inputs.outputtype.checked_push(txinindex, outputtype)?;
        inputs.typeindex.checked_push(txinindex, typeindex)?;

        if !outputtype.is_address() {
            continue;
        }
        let addresstype = outputtype;
        let addressindex = typeindex;

        addr_txindex_stores
            .get_mut_unwrap(addresstype)
            .insert(AddressIndexTxIndex::from((addressindex, txindex)), Unit);

        addr_outpoint_stores
            .get_mut_unwrap(addresstype)
            .remove(AddressIndexOutPoint::from((addressindex, outpoint)));
    }

    Ok(())
}
