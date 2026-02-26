use brk_cohort::ByAddressType;
use brk_error::{Error, Result};
use brk_store::Store;
use brk_types::{
    AddressBytes, AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, OutPoint, OutputType,
    Sats, TxIndex, TxOutIndex, TypeIndex, Unit, Vout,
};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::error;
use vecdb::{BytesVec, WritableVec};

use super::{BlockProcessor, ProcessedOutput, SameBlockOutputInfo};
use crate::{AddressesVecs, Indexes, OutputsVecs, ScriptsVecs};

impl<'a> BlockProcessor<'a> {
    pub fn process_outputs(&self) -> Result<Vec<ProcessedOutput<'a>>> {
        let height = self.height;
        let check_collisions = self.check_collisions;

        let base_txindex = self.indexes.txindex;
        let base_txoutindex = self.indexes.txoutindex;

        let total_outputs: usize = self.block.txdata.iter().map(|tx| tx.output.len()).sum();
        let mut items = Vec::with_capacity(total_outputs);
        for (index, tx) in self.block.txdata.iter().enumerate() {
            for (vout, txout) in tx.output.iter().enumerate() {
                items.push((TxIndex::from(index), Vout::from(vout), txout));
            }
        }

        items
            .into_par_iter()
            .enumerate()
            .map(
                |(block_txoutindex, (block_txindex, vout, txout))| -> Result<ProcessedOutput> {
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
                        .get(&address_hash)?
                        .map(|v| *v)
                        .and_then(|typeindex_local| {
                            (typeindex_local < self.indexes.to_typeindex(addresstype))
                                .then_some(typeindex_local)
                        });

                    if check_collisions && let Some(typeindex) = existing_typeindex {
                        let prev_addressbytes = self
                            .vecs
                            .addresses
                            .get_bytes_by_type(addresstype, typeindex, &self.readers.addressbytes)
                            .ok_or(Error::Internal("Missing addressbytes"))?;

                        if prev_addressbytes != address_bytes {
                            error!(
                                ?height,
                                ?vout,
                                ?block_txindex,
                                ?addresstype,
                                ?prev_addressbytes,
                                ?address_bytes,
                                ?typeindex,
                                "Address hash collision"
                            );
                            return Err(Error::Internal("Address hash collision"));
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
}

#[allow(clippy::too_many_arguments)]
pub(super) fn finalize_outputs(
    indexes: &mut Indexes,
    first_txoutindex: &mut BytesVec<TxIndex, TxOutIndex>,
    outputs: &mut OutputsVecs,
    addresses: &mut AddressesVecs,
    scripts: &mut ScriptsVecs,
    addr_hash_stores: &mut ByAddressType<Store<AddressHash, TypeIndex>>,
    addr_txindex_stores: &mut ByAddressType<Store<AddressIndexTxIndex, Unit>>,
    addr_outpoint_stores: &mut ByAddressType<Store<AddressIndexOutPoint, Unit>>,
    txouts: Vec<ProcessedOutput>,
    same_block_spent_outpoints: &FxHashSet<OutPoint>,
    already_added_addresshash: &mut ByAddressType<FxHashMap<AddressHash, TypeIndex>>,
    same_block_output_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
) -> Result<()> {
    already_added_addresshash
        .values_mut()
        .for_each(|m| m.clear());
    same_block_output_info.clear();

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
            first_txoutindex.checked_push(txindex, txoutindex)?;
        }

        outputs.txindex.checked_push(txoutindex, txindex)?;

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
                let ti = indexes.increment_address_index(addresstype);

                already_added_addresshash
                    .get_mut_unwrap(addresstype)
                    .insert(address_hash, ti);
                addr_hash_stores
                    .get_mut_unwrap(addresstype)
                    .insert(address_hash, ti);
                addresses.push_bytes_if_needed(ti, address_bytes)?;

                ti
            }
        } else {
            match outputtype {
                OutputType::P2MS => {
                    scripts
                        .p2ms_to_txindex
                        .checked_push(indexes.p2msoutputindex, txindex)?;
                    indexes.p2msoutputindex.copy_then_increment()
                }
                OutputType::OpReturn => {
                    scripts
                        .opreturn_to_txindex
                        .checked_push(indexes.opreturnindex, txindex)?;
                    indexes.opreturnindex.copy_then_increment()
                }
                OutputType::Empty => {
                    scripts
                        .empty_to_txindex
                        .checked_push(indexes.emptyoutputindex, txindex)?;
                    indexes.emptyoutputindex.copy_then_increment()
                }
                OutputType::Unknown => {
                    scripts
                        .unknown_to_txindex
                        .checked_push(indexes.unknownoutputindex, txindex)?;
                    indexes.unknownoutputindex.copy_then_increment()
                }
                _ => unreachable!(),
            }
        };

        outputs.value.checked_push(txoutindex, sats)?;
        outputs.outputtype.checked_push(txoutindex, outputtype)?;
        outputs.typeindex.checked_push(txoutindex, typeindex)?;

        if outputtype.is_unspendable() {
            continue;
        } else if outputtype.is_address() {
            let addresstype = outputtype;
            let addressindex = typeindex;

            addr_txindex_stores
                .get_mut_unwrap(addresstype)
                .insert(AddressIndexTxIndex::from((addressindex, txindex)), Unit);
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

            addr_outpoint_stores
                .get_mut_unwrap(addresstype)
                .insert(AddressIndexOutPoint::from((addressindex, outpoint)), Unit);
        }
    }

    Ok(())
}
