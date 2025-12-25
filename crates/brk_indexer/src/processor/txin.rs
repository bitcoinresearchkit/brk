//! Input processing for block indexing.

use brk_error::{Error, Result};
use brk_types::{
    AddressIndexOutPoint, AddressIndexTxIndex, OutPoint, OutputType, Sats, TxInIndex, TxIndex,
    TxOutIndex, Txid, TxidPrefix, TypeIndex, Unit, Vin, Vout,
};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::GenericStoredVec;

use super::{BlockProcessor, ComputedTx, InputSource, SameBlockOutputInfo};

impl<'a> BlockProcessor<'a> {
    /// Process inputs in parallel.
    ///
    /// Uses collect().into_par_iter() pattern because:
    /// 1. The inner work (store lookups, vector reads) is expensive
    /// 2. We want to parallelize across ALL inputs, not just per-transaction
    /// 3. The intermediate allocation (~8KB per block) is negligible compared to parallelism gains
    pub fn process_inputs<'c>(
        &self,
        txs: &[ComputedTx<'c>],
    ) -> Result<Vec<(TxInIndex, InputSource<'a>)>> {
        let txid_prefix_to_txindex: FxHashMap<_, _> =
            txs.iter().map(|ct| (ct.txid_prefix, &ct.txindex)).collect();

        let base_txindex = self.indexes.txindex;
        let base_txinindex = self.indexes.txinindex;

        let txins = self
            .block
            .txdata
            .iter()
            .enumerate()
            .flat_map(|(index, tx)| {
                tx.input
                    .iter()
                    .enumerate()
                    .map(move |(vin, txin)| (TxIndex::from(index), Vin::from(vin), txin, tx))
            })
            .collect::<Vec<_>>()
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
                                txin,
                                vin,
                                outpoint: OutPoint::COINBASE,
                            },
                        ));
                    }

                    let outpoint = txin.previous_output;
                    let txid = Txid::from(outpoint.txid);
                    let txid_prefix = TxidPrefix::from(&txid);
                    let vout = Vout::from(outpoint.vout);

                    if let Some(&&same_block_txindex) = txid_prefix_to_txindex
                        .get(&txid_prefix) {
                        let outpoint = OutPoint::new(same_block_txindex, vout);
                        return Ok((
                            txinindex,
                            InputSource::SameBlock {
                                txindex,
                                txin,
                                vin,
                                outpoint,
                            },
                        ));
                    }

                    let prev_txindex = if let Some(txindex) = self
                        .stores
                        .txidprefix_to_txindex
                        .get(&txid_prefix)?
                        .map(|v| *v)
                        .and_then(|txindex| {
                            (txindex < self.indexes.txindex).then_some(txindex)
                        })
                    {
                        txindex
                    } else {
                        return Err(Error::UnknownTxid);
                    };

                    let txoutindex = self
                        .vecs
                        .tx
                        .txindex_to_first_txoutindex
                        .get_pushed_or_read(prev_txindex, &self.readers.txindex_to_first_txoutindex)?
                        .ok_or(Error::Internal("Missing txoutindex"))?
                        + vout;

                    let outpoint = OutPoint::new(prev_txindex, vout);

                    let txoutdata = self
                        .vecs
                        .txout
                        .txoutindex_to_txoutdata
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_txoutdata)?
                        .ok_or(Error::Internal("Missing txout data"))?;

                    let value = txoutdata.value;
                    let outputtype = txoutdata.outputtype;
                    let typeindex = txoutdata.typeindex;

                    let height = self
                        .txindex_to_height
                        .get(prev_txindex)
                        .ok_or(Error::Internal("Missing height in txindex_to_height map"))?;

                    Ok((
                        txinindex,
                        InputSource::PreviousBlock {
                            vin,
                            value,
                            height,
                            txindex,
                            txoutindex,
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

    /// Collect same-block spent outpoints.
    pub fn collect_same_block_spent_outpoints(
        txins: &[(TxInIndex, InputSource)],
    ) -> FxHashSet<OutPoint> {
        txins
            .iter()
            .filter_map(|(_, input_source)| {
                let InputSource::SameBlock { outpoint, .. } = input_source else {
                    return None;
                };
                if !outpoint.is_coinbase() {
                    Some(*outpoint)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Finalize inputs sequentially (stores outpoints, updates address UTXOs).
    pub fn finalize_inputs(
        &mut self,
        txins: Vec<(TxInIndex, InputSource)>,
        same_block_output_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
    ) -> Result<()> {
        let height = self.height;

        for (txinindex, input_source) in txins {
            let (prev_height, vin, txindex, value, outpoint, txoutindex, outputtype, typeindex) =
                match input_source {
                    InputSource::PreviousBlock {
                        height,
                        vin,
                        txindex,
                        txoutindex,
                        value,
                        outpoint,
                        outputtype,
                        typeindex,
                    } => (
                        height, vin, txindex, value, outpoint, txoutindex, outputtype, typeindex,
                    ),
                    InputSource::SameBlock {
                        txindex,
                        txin,
                        vin,
                        outpoint,
                    } => {
                        if outpoint.is_coinbase() {
                            (
                                height,
                                vin,
                                txindex,
                                Sats::COINBASE,
                                outpoint,
                                TxOutIndex::COINBASE,
                                OutputType::Unknown,
                                TypeIndex::COINBASE,
                            )
                        } else {
                            let info = same_block_output_info
                                .remove(&outpoint)
                                .ok_or(Error::Internal("Same-block output not found"))
                                .inspect_err(|_| {
                                    dbg!(&same_block_output_info, txin);
                                })?;
                            (
                                height,
                                vin,
                                txindex,
                                info.value,
                                outpoint,
                                info.txoutindex,
                                info.outputtype,
                                info.typeindex,
                            )
                        }
                    }
                };

            if vin.is_zero() {
                self.vecs
                    .tx
                    .txindex_to_first_txinindex
                    .checked_push(txindex, txinindex)?;
            }

            self.vecs
                .txin
                .txinindex_to_txindex
                .checked_push(txinindex, txindex)?;
            self.vecs
                .txin
                .txinindex_to_outpoint
                .checked_push(txinindex, outpoint)?;
            self.vecs
                .txin
                .txinindex_to_value
                .checked_push(txinindex, value)?;
            self.vecs
                .txin
                .txinindex_to_prev_height
                .checked_push(txinindex, prev_height)?;
            self.vecs
                .txin
                .txinindex_to_outputtype
                .checked_push(txinindex, outputtype)?;
            self.vecs
                .txin
                .txinindex_to_typeindex
                .checked_push(txinindex, typeindex)?;

            // Update txoutindex_to_txinindex for non-coinbase inputs
            if !txoutindex.is_coinbase() {
                self.vecs
                    .txout
                    .txoutindex_to_txinindex
                    .update(txoutindex, txinindex)?;
            }

            if !outputtype.is_address() {
                continue;
            }
            let addresstype = outputtype;
            let addressindex = typeindex;

            self.stores
                .addresstype_to_addressindex_and_txindex
                .get_mut_unwrap(addresstype)
                .insert_if_needed(
                    AddressIndexTxIndex::from((addressindex, txindex)),
                    Unit,
                    height,
                );

            self.stores
                .addresstype_to_addressindex_and_unspentoutpoint
                .get_mut_unwrap(addresstype)
                .remove_if_needed(AddressIndexOutPoint::from((addressindex, outpoint)), height);
        }

        Ok(())
    }
}
