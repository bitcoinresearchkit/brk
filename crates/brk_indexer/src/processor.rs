use bitcoin::{Transaction, TxIn, TxOut};
use brk_error::{Error, Result};
use brk_grouper::ByAddressType;
use brk_types::{
    AddressBytes, AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, Block, BlockHashPrefix,
    Height, OutPoint, OutputType, Sats, StoredBool, Timestamp, TxInIndex, TxIndex, TxOutIndex,
    Txid, TxidPrefix, TypeIndex, Unit, Vin, Vout,
};
use log::error;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::{AnyVec, GenericStoredVec, TypedVecIterator};

use crate::{Indexes, Readers, Stores, Vecs, constants::*};

/// Input source for tracking where an input came from.
#[derive(Debug)]
pub enum InputSource<'a> {
    PreviousBlock {
        vin: Vin,
        txindex: TxIndex,
        outpoint: OutPoint,
        address_info: Option<(OutputType, TypeIndex)>,
    },
    SameBlock {
        txindex: TxIndex,
        txin: &'a TxIn,
        vin: Vin,
        outpoint: OutPoint,
    },
}

/// Processed output data from parallel output processing.
pub struct ProcessedOutput<'a> {
    pub txoutindex: TxOutIndex,
    pub txout: &'a TxOut,
    pub txindex: TxIndex,
    pub vout: Vout,
    pub outputtype: OutputType,
    pub address_info: Option<(AddressBytes, AddressHash)>,
    pub existing_typeindex: Option<TypeIndex>,
}

/// Computed transaction data from parallel TXID computation.
pub struct ComputedTx<'a> {
    pub txindex: TxIndex,
    pub tx: &'a Transaction,
    pub txid: Txid,
    pub txid_prefix: TxidPrefix,
    pub prev_txindex_opt: Option<TxIndex>,
}

/// Processes a single block, extracting and storing all indexed data.
pub struct BlockProcessor<'a> {
    pub block: &'a Block,
    pub height: Height,
    pub check_collisions: bool,
    pub indexes: &'a mut Indexes,
    pub vecs: &'a mut Vecs,
    pub stores: &'a mut Stores,
    pub readers: &'a Readers,
}

impl<'a> BlockProcessor<'a> {
    /// Process block metadata (blockhash, difficulty, timestamp, etc.)
    pub fn process_block_metadata(&mut self) -> Result<()> {
        let height = self.height;
        let blockhash = self.block.hash();
        let blockhash_prefix = BlockHashPrefix::from(blockhash);

        // Check for blockhash prefix collision
        if self
            .stores
            .blockhashprefix_to_height
            .get(&blockhash_prefix)?
            .is_some_and(|prev_height| *prev_height != height)
        {
            error!("BlockHash: {blockhash}");
            return Err(Error::Internal("BlockHash prefix collision"));
        }

        self.indexes.checked_push(self.vecs)?;

        self.stores
            .blockhashprefix_to_height
            .insert_if_needed(blockhash_prefix, height, height);

        self.stores.height_to_coinbase_tag.insert_if_needed(
            height,
            self.block.coinbase_tag().into(),
            height,
        );

        self.vecs
            .block
            .height_to_blockhash
            .checked_push(height, blockhash.clone())?;
        self.vecs
            .block
            .height_to_difficulty
            .checked_push(height, self.block.header.difficulty_float().into())?;
        self.vecs
            .block
            .height_to_timestamp
            .checked_push(height, Timestamp::from(self.block.header.time))?;
        self.vecs
            .block
            .height_to_total_size
            .checked_push(height, self.block.total_size().into())?;
        self.vecs
            .block
            .height_to_weight
            .checked_push(height, self.block.weight().into())?;

        Ok(())
    }

    /// Compute TXIDs in parallel (CPU-intensive operation).
    pub fn compute_txids(&self) -> Result<Vec<ComputedTx<'a>>> {
        let will_check_collisions =
            self.check_collisions && self.stores.txidprefix_to_txindex.needs(self.height);
        let base_txindex = self.indexes.txindex;

        self.block
            .txdata
            .par_iter()
            .enumerate()
            .map(|(index, tx)| {
                let txid = Txid::from(tx.compute_txid());
                let txid_prefix = TxidPrefix::from(&txid);

                let prev_txindex_opt = if will_check_collisions {
                    self.stores
                        .txidprefix_to_txindex
                        .get(&txid_prefix)?
                        .map(|v| *v)
                } else {
                    None
                };

                Ok(ComputedTx {
                    txindex: base_txindex + TxIndex::from(index),
                    tx,
                    txid,
                    txid_prefix,
                    prev_txindex_opt,
                })
            })
            .collect()
    }

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
                    let outputtype = self
                        .vecs
                        .txout
                        .txoutindex_to_outputtype
                        .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_outputtype)?
                        .ok_or(Error::Internal("Missing outputtype"))?;

                    let address_info = if outputtype.is_address() {
                        let typeindex = self
                            .vecs
                            .txout
                            .txoutindex_to_typeindex
                            .get_pushed_or_read(txoutindex, &self.readers.txoutindex_to_typeindex)?
                            .ok_or(Error::Internal("Missing typeindex"))?;
                        Some((outputtype, typeindex))
                    } else {
                        None
                    };

                    Ok((
                        txinindex,
                        InputSource::PreviousBlock {
                            vin,
                            txindex,
                            outpoint,
                            address_info,
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

    /// Process outputs in parallel.
    pub fn process_outputs(&self) -> Result<Vec<ProcessedOutput<'a>>> {
        let height = self.height;
        let check_collisions = self.check_collisions;

        let base_txindex = self.indexes.txindex;
        let base_txoutindex = self.indexes.txoutindex;

        // Same pattern as inputs: collect then parallelize for maximum parallelism
        self.block
            .txdata
            .iter()
            .enumerate()
            .flat_map(|(index, tx)| {
                tx.output
                    .iter()
                    .enumerate()
                    .map(move |(vout, txout)| (TxIndex::from(index), Vout::from(vout), txout, tx))
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .enumerate()
            .map(
                |(block_txoutindex, (block_txindex, vout, txout, tx))| -> Result<ProcessedOutput> {
                    let txindex = base_txindex + block_txindex;
                    let txoutindex = base_txoutindex + TxOutIndex::from(block_txoutindex);

                    let script = &txout.script_pubkey;
                    let outputtype = OutputType::from(script);

                    if outputtype.is_not_address() {
                        return Ok(ProcessedOutput {
                            txoutindex,
                            txout,
                            txindex,
                            vout,
                            outputtype,
                            address_info: None,
                            existing_typeindex: None,
                        });
                    }

                    let addresstype = outputtype;
                    let address_bytes = AddressBytes::try_from((script, addresstype)).unwrap();
                    let address_hash = AddressHash::from(&address_bytes);

                    let existing_typeindex = self
                        .stores
                        .addresstype_to_addresshash_to_addressindex
                        .get_unwrap(addresstype)
                        .get(&address_hash)
                        .unwrap()
                        .map(|v| *v)
                        .and_then(|typeindex_local| {
                            (typeindex_local < self.indexes.to_typeindex(addresstype))
                                .then_some(typeindex_local)
                        });

                    if check_collisions && let Some(typeindex) = existing_typeindex {
                        let prev_addressbytes_opt = self.vecs.get_addressbytes_by_type(
                            addresstype,
                            typeindex,
                            self.readers.addressbytes.get_unwrap(addresstype),
                        )?;
                        let prev_addressbytes = prev_addressbytes_opt
                            .as_ref()
                            .ok_or(Error::Internal("Missing addressbytes"))?;

                        if self
                            .stores
                            .addresstype_to_addresshash_to_addressindex
                            .get_unwrap(addresstype)
                            .needs(height)
                            && prev_addressbytes != &address_bytes
                        {
                            let txid = tx.compute_txid();
                            dbg!(
                                height,
                                txid,
                                vout,
                                block_txindex,
                                addresstype,
                                prev_addressbytes,
                                &address_bytes,
                                &self.indexes,
                                typeindex,
                                txout,
                                AddressHash::from(&address_bytes),
                            );
                            panic!()
                        }
                    }

                    Ok(ProcessedOutput {
                        txoutindex,
                        txout,
                        txindex,
                        vout,
                        outputtype,
                        address_info: Some((address_bytes, address_hash)),
                        existing_typeindex,
                    })
                },
            )
            .collect()
    }

    /// Finalize outputs sequentially (stores addresses, tracks UTXOs).
    pub fn finalize_outputs(
        &mut self,
        txouts: Vec<ProcessedOutput>,
        same_block_spent_outpoints: &FxHashSet<OutPoint>,
    ) -> Result<FxHashMap<OutPoint, (OutputType, TypeIndex)>> {
        let height = self.height;
        let mut already_added_addresshash: ByAddressType<FxHashMap<AddressHash, TypeIndex>> =
            ByAddressType::default();
        // Pre-size based on the number of same-block spent outpoints
        let mut same_block_output_info: FxHashMap<OutPoint, (OutputType, TypeIndex)> =
            FxHashMap::with_capacity_and_hasher(
                same_block_spent_outpoints.len(),
                Default::default(),
            );

        for ProcessedOutput {
            txoutindex,
            txout,
            txindex,
            vout,
            outputtype,
            address_info,
            existing_typeindex,
        } in txouts
        {
            let sats = Sats::from(txout.value);

            if vout.is_zero() {
                self.vecs
                    .tx
                    .txindex_to_first_txoutindex
                    .checked_push(txindex, txoutindex)?;
            }

            self.vecs
                .txout
                .txoutindex_to_value
                .checked_push(txoutindex, sats)?;
            self.vecs
                .txout
                .txoutindex_to_txindex
                .checked_push(txoutindex, txindex)?;
            self.vecs
                .txout
                .txoutindex_to_outputtype
                .checked_push(txoutindex, outputtype)?;

            let typeindex = if let Some(ti) = existing_typeindex {
                ti
            } else if let Some((address_bytes, address_hash)) = address_info {
                let addresstype = outputtype;
                if let Some(&ti) = already_added_addresshash
                    .get_unwrap(addresstype)
                    .get(&address_hash)
                {
                    ti
                } else {
                    let ti = self.indexes.increment_address_index(addresstype);

                    already_added_addresshash
                        .get_mut_unwrap(addresstype)
                        .insert(address_hash, ti);
                    self.stores
                        .addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(addresstype)
                        .insert_if_needed(address_hash, ti, height);
                    self.vecs.push_bytes_if_needed(ti, address_bytes)?;

                    ti
                }
            } else {
                match outputtype {
                    OutputType::P2MS => {
                        self.vecs
                            .output
                            .p2msoutputindex_to_txindex
                            .checked_push(self.indexes.p2msoutputindex, txindex)?;
                        self.indexes.p2msoutputindex.copy_then_increment()
                    }
                    OutputType::OpReturn => {
                        self.vecs
                            .output
                            .opreturnindex_to_txindex
                            .checked_push(self.indexes.opreturnindex, txindex)?;
                        self.indexes.opreturnindex.copy_then_increment()
                    }
                    OutputType::Empty => {
                        self.vecs
                            .output
                            .emptyoutputindex_to_txindex
                            .checked_push(self.indexes.emptyoutputindex, txindex)?;
                        self.indexes.emptyoutputindex.copy_then_increment()
                    }
                    OutputType::Unknown => {
                        self.vecs
                            .output
                            .unknownoutputindex_to_txindex
                            .checked_push(self.indexes.unknownoutputindex, txindex)?;
                        self.indexes.unknownoutputindex.copy_then_increment()
                    }
                    _ => unreachable!(),
                }
            };

            self.vecs
                .txout
                .txoutindex_to_typeindex
                .checked_push(txoutindex, typeindex)?;

            if outputtype.is_unspendable() {
                continue;
            } else if outputtype.is_address() {
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
            }

            let outpoint = OutPoint::new(txindex, vout);

            if same_block_spent_outpoints.contains(&outpoint) {
                same_block_output_info.insert(outpoint, (outputtype, typeindex));
            } else if outputtype.is_address() {
                let addresstype = outputtype;
                let addressindex = typeindex;

                self.stores
                    .addresstype_to_addressindex_and_unspentoutpoint
                    .get_mut_unwrap(addresstype)
                    .insert_if_needed(
                        AddressIndexOutPoint::from((addressindex, outpoint)),
                        Unit,
                        height,
                    );
            }
        }

        Ok(same_block_output_info)
    }

    /// Finalize inputs sequentially (stores outpoints, updates address UTXOs).
    pub fn finalize_inputs(
        &mut self,
        txins: Vec<(TxInIndex, InputSource)>,
        same_block_output_info: &mut FxHashMap<OutPoint, (OutputType, TypeIndex)>,
    ) -> Result<()> {
        let height = self.height;

        for (txinindex, input_source) in txins {
            let (vin, txindex, outpoint, address_info) = match input_source {
                InputSource::PreviousBlock {
                    vin,
                    txindex,
                    outpoint,
                    address_info,
                } => (vin, txindex, outpoint, address_info),
                InputSource::SameBlock {
                    txindex,
                    txin,
                    vin,
                    outpoint,
                } => {
                    if outpoint.is_coinbase() {
                        (vin, txindex, outpoint, None)
                    } else {
                        let outputtype_typeindex = same_block_output_info
                            .remove(&outpoint)
                            .ok_or(Error::Internal("Same-block addressindex not found"))
                            .inspect_err(|_| {
                                dbg!(&same_block_output_info, txin);
                            })?;
                        let address_info = if outputtype_typeindex.0.is_address() {
                            Some(outputtype_typeindex)
                        } else {
                            None
                        };
                        (vin, txindex, outpoint, address_info)
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

            let Some((addresstype, addressindex)) = address_info else {
                continue;
            };

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

    /// Check for TXID collisions (only for known duplicate TXIDs).
    pub fn check_txid_collisions(&self, txs: &[ComputedTx]) -> Result<()> {
        if !self.check_collisions {
            return Ok(());
        }

        let mut txindex_to_txid_iter = self.vecs.tx.txindex_to_txid.into_iter();
        for ct in txs.iter() {
            let Some(prev_txindex) = ct.prev_txindex_opt else {
                continue;
            };

            // In case if we start at an already parsed height
            if ct.txindex == prev_txindex {
                continue;
            }

            let len = self.vecs.tx.txindex_to_txid.len();
            let prev_txid = txindex_to_txid_iter
                .get(prev_txindex)
                .ok_or(Error::Internal("Missing txid for txindex"))
                .inspect_err(|_| {
                    dbg!(ct.txindex, len);
                })?;

            let is_dup = DUPLICATE_TXIDS.contains(&prev_txid);

            if !is_dup {
                dbg!(self.height, ct.txindex, prev_txid, prev_txindex);
                return Err(Error::Internal("Unexpected TXID collision"));
            }
        }

        Ok(())
    }

    /// Store transaction metadata.
    pub fn store_transaction_metadata(&mut self, txs: Vec<ComputedTx>) -> Result<()> {
        let height = self.height;

        for ct in txs {
            if ct.prev_txindex_opt.is_none() {
                self.stores.txidprefix_to_txindex.insert_if_needed(
                    ct.txid_prefix,
                    ct.txindex,
                    height,
                );
            }

            self.vecs
                .tx
                .txindex_to_height
                .checked_push(ct.txindex, height)?;
            self.vecs
                .tx
                .txindex_to_txversion
                .checked_push(ct.txindex, ct.tx.version.into())?;
            self.vecs
                .tx
                .txindex_to_txid
                .checked_push(ct.txindex, ct.txid)?;
            self.vecs
                .tx
                .txindex_to_rawlocktime
                .checked_push(ct.txindex, ct.tx.lock_time.into())?;
            self.vecs
                .tx
                .txindex_to_base_size
                .checked_push(ct.txindex, ct.tx.base_size().into())?;
            self.vecs
                .tx
                .txindex_to_total_size
                .checked_push(ct.txindex, ct.tx.total_size().into())?;
            self.vecs
                .tx
                .txindex_to_is_explicitly_rbf
                .checked_push(ct.txindex, StoredBool::from(ct.tx.is_explicitly_rbf()))?;
        }

        Ok(())
    }

    /// Update global indexes after processing a block.
    pub fn update_indexes(&mut self, tx_count: usize, input_count: usize, output_count: usize) {
        self.indexes.txindex += TxIndex::from(tx_count);
        self.indexes.txinindex += TxInIndex::from(input_count);
        self.indexes.txoutindex += TxOutIndex::from(output_count);
    }
}
