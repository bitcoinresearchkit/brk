use brk_error::{Error, Result};
use brk_grouper::ByAddressType;
use brk_types::{
    AddressBytes, AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, OutPoint, OutputType,
    Sats, TxIndex, TxOutIndex, TypeIndex, Unit, Vout,
};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::GenericStoredVec;

use super::{BlockProcessor, ProcessedOutput, SameBlockOutputInfo};

impl<'a> BlockProcessor<'a> {
    pub fn process_outputs(&self) -> Result<Vec<ProcessedOutput<'a>>> {
        let height = self.height;
        let check_collisions = self.check_collisions;

        let base_txindex = self.indexes.txindex;
        let base_txoutindex = self.indexes.txoutindex;

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

    pub fn finalize_outputs(
        &mut self,
        txouts: Vec<ProcessedOutput>,
        same_block_spent_outpoints: &FxHashSet<OutPoint>,
    ) -> Result<FxHashMap<OutPoint, SameBlockOutputInfo>> {
        let height = self.height;
        let mut already_added_addresshash: ByAddressType<FxHashMap<AddressHash, TypeIndex>> =
            ByAddressType::default();
        let mut same_block_output_info: FxHashMap<OutPoint, SameBlockOutputInfo> =
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
                .txoutindex_to_txindex
                .checked_push(txoutindex, txindex)?;

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
                .txoutindex_to_value
                .checked_push(txoutindex, sats)?;
            self.vecs
                .txout
                .txoutindex_to_outputtype
                .checked_push(txoutindex, outputtype)?;
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
                same_block_output_info.insert(
                    outpoint,
                    SameBlockOutputInfo {
                        outputtype,
                        typeindex,
                    },
                );
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
}
