use brk_error::{Error, Result};
use brk_types::{
    AddressIndexOutPoint, AddressIndexTxIndex, OutPoint, OutputType, TxInIndex, TxIndex, Txid,
    TxidPrefix, TypeIndex, Unit, Vin, Vout,
};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::GenericStoredVec;

use super::{BlockProcessor, ComputedTx, InputSource, SameBlockOutputInfo};

impl<'a> BlockProcessor<'a> {
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
                        .transactions
                        .first_txoutindex
                        .get_pushed_or_read(prev_txindex, &self.readers.txindex_to_first_txoutindex)?
                        .ok_or(Error::Internal("Missing txoutindex"))?
                        + vout;

                    let outpoint = OutPoint::new(prev_txindex, vout);

                    let outputtype = self
                        .vecs
                        .outputs
                        .outputtype
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_outputtype)?
                        .ok_or(Error::Internal("Missing outputtype"))?;

                    let typeindex = self
                        .vecs
                        .outputs
                        .typeindex
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_typeindex)?
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

    pub fn finalize_inputs(
        &mut self,
        txins: Vec<(TxInIndex, InputSource)>,
        mut same_block_output_info: FxHashMap<OutPoint, SameBlockOutputInfo>,
    ) -> Result<()> {
        let height = self.height;

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
                    txin,
                    vin,
                    outpoint,
                } => {
                    if outpoint.is_coinbase() {
                        (vin, txindex, outpoint, OutputType::Unknown, TypeIndex::COINBASE)
                    } else {
                        let info = same_block_output_info
                            .remove(&outpoint)
                            .ok_or(Error::Internal("Same-block output not found"))
                            .inspect_err(|_| {
                                dbg!(&same_block_output_info, txin);
                            })?;
                        (vin, txindex, outpoint, info.outputtype, info.typeindex)
                    }
                }
            };

            if vin.is_zero() {
                self.vecs
                    .transactions
                    .first_txinindex
                    .checked_push(txindex, txinindex)?;
            }

            self.vecs
                .inputs
                .txindex
                .checked_push(txinindex, txindex)?;
            self.vecs
                .inputs
                .outpoint
                .checked_push(txinindex, outpoint)?;
            self.vecs
                .inputs
                .outputtype
                .checked_push(txinindex, outputtype)?;
            self.vecs
                .inputs
                .typeindex
                .checked_push(txinindex, typeindex)?;

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
