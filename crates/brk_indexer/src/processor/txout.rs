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

        let base_tx_index = self.indexes.tx_index;
        let base_txout_index = self.indexes.txout_index;

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
                |(block_txout_index, (block_tx_index, vout, txout))| -> Result<ProcessedOutput> {
                    let tx_index = base_tx_index + block_tx_index;
                    let txout_index = base_txout_index + TxOutIndex::from(block_txout_index);

                    let script = &txout.script_pubkey;
                    let output_type = OutputType::from(script);

                    if output_type.is_not_address() {
                        return Ok(ProcessedOutput {
                            txout_index,
                            txout,
                            tx_index,
                            vout,
                            output_type,
                            address_info: None,
                            existing_type_index: None,
                        });
                    }

                    let address_type = output_type;
                    let address_bytes = AddressBytes::try_from((script, address_type)).unwrap();
                    let address_hash = AddressHash::from(&address_bytes);

                    let existing_type_index = self
                        .stores
                        .address_type_to_address_hash_to_address_index
                        .get_unwrap(address_type)
                        .get(&address_hash)?
                        .map(|v| *v)
                        .and_then(|type_index_local| {
                            (type_index_local < self.indexes.to_type_index(address_type))
                                .then_some(type_index_local)
                        });

                    if check_collisions && let Some(type_index) = existing_type_index {
                        let prev_addressbytes = self
                            .vecs
                            .addresses
                            .get_bytes_by_type(address_type, type_index, &self.readers.addressbytes)
                            .ok_or(Error::Internal("Missing addressbytes"))?;

                        if prev_addressbytes != address_bytes {
                            error!(
                                ?height,
                                ?vout,
                                ?block_tx_index,
                                ?address_type,
                                ?prev_addressbytes,
                                ?address_bytes,
                                ?type_index,
                                "Address hash collision"
                            );
                            return Err(Error::Internal("Address hash collision"));
                        }
                    }

                    Ok(ProcessedOutput {
                        txout_index,
                        txout,
                        tx_index,
                        vout,
                        output_type,
                        address_info: Some((address_bytes, address_hash)),
                        existing_type_index,
                    })
                },
            )
            .collect()
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn finalize_outputs(
    indexes: &mut Indexes,
    first_txout_index: &mut BytesVec<TxIndex, TxOutIndex>,
    outputs: &mut OutputsVecs,
    addresses: &mut AddressesVecs,
    scripts: &mut ScriptsVecs,
    addr_hash_stores: &mut ByAddressType<Store<AddressHash, TypeIndex>>,
    addr_tx_index_stores: &mut ByAddressType<Store<AddressIndexTxIndex, Unit>>,
    addr_outpoint_stores: &mut ByAddressType<Store<AddressIndexOutPoint, Unit>>,
    txouts: Vec<ProcessedOutput>,
    same_block_spent_outpoints: &FxHashSet<OutPoint>,
    already_added_address_hash: &mut ByAddressType<FxHashMap<AddressHash, TypeIndex>>,
    same_block_output_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
) -> Result<()> {
    already_added_address_hash
        .values_mut()
        .for_each(|m| m.clear());
    same_block_output_info.clear();

    for ProcessedOutput {
        txout_index,
        txout,
        tx_index,
        vout,
        output_type,
        address_info,
        existing_type_index,
    } in txouts
    {
        let sats = Sats::from(txout.value);

        if vout.is_zero() {
            first_txout_index.checked_push(tx_index, txout_index)?;
        }

        outputs.tx_index.checked_push(txout_index, tx_index)?;

        let type_index = if let Some(ti) = existing_type_index {
            ti
        } else if let Some((address_bytes, address_hash)) = address_info {
            let address_type = output_type;
            if let Some(&ti) = already_added_address_hash
                .get_unwrap(address_type)
                .get(&address_hash)
            {
                ti
            } else {
                let ti = indexes.increment_address_index(address_type);

                already_added_address_hash
                    .get_mut_unwrap(address_type)
                    .insert(address_hash, ti);
                addr_hash_stores
                    .get_mut_unwrap(address_type)
                    .insert(address_hash, ti);
                addresses.push_bytes_if_needed(ti, address_bytes)?;

                ti
            }
        } else {
            match output_type {
                OutputType::P2MS => {
                    scripts
                        .p2ms.to_tx_index
                        .checked_push(indexes.p2ms_output_index, tx_index)?;
                    indexes.p2ms_output_index.copy_then_increment()
                }
                OutputType::OpReturn => {
                    scripts
                        .op_return.to_tx_index
                        .checked_push(indexes.op_return_index, tx_index)?;
                    indexes.op_return_index.copy_then_increment()
                }
                OutputType::Empty => {
                    scripts
                        .empty.to_tx_index
                        .checked_push(indexes.empty_output_index, tx_index)?;
                    indexes.empty_output_index.copy_then_increment()
                }
                OutputType::Unknown => {
                    scripts
                        .unknown.to_tx_index
                        .checked_push(indexes.unknown_output_index, tx_index)?;
                    indexes.unknown_output_index.copy_then_increment()
                }
                _ => unreachable!(),
            }
        };

        outputs.value.checked_push(txout_index, sats)?;
        outputs.output_type.checked_push(txout_index, output_type)?;
        outputs.type_index.checked_push(txout_index, type_index)?;

        if output_type.is_unspendable() {
            continue;
        } else if output_type.is_address() {
            let address_type = output_type;
            let address_index = type_index;

            addr_tx_index_stores
                .get_mut_unwrap(address_type)
                .insert(AddressIndexTxIndex::from((address_index, tx_index)), Unit);
        }

        let outpoint = OutPoint::new(tx_index, vout);

        if same_block_spent_outpoints.contains(&outpoint) {
            same_block_output_info.insert(
                outpoint,
                SameBlockOutputInfo {
                    output_type,
                    type_index,
                },
            );
        } else if output_type.is_address() {
            let address_type = output_type;
            let address_index = type_index;

            addr_outpoint_stores
                .get_mut_unwrap(address_type)
                .insert(AddressIndexOutPoint::from((address_index, outpoint)), Unit);
        }
    }

    Ok(())
}
