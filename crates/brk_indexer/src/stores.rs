use std::{fs, path::Path, time::Instant};

use rustc_hash::FxHashSet;

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_store::{AnyStore, Kind, Mode, Store};
use brk_types::{
    AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, BlockHashPrefix, Height, OutPoint,
    OutputType, StoredString, TxIndex, TxOutIndex, TxidPrefix, TypeIndex, Unit, Version, Vout,
};
use fjall::{Database, PersistMode};
use rayon::prelude::*;
use tracing::info;
use vecdb::{AnyVec, TypedVecIterator, VecIndex, VecIterator};

use crate::{Indexes, constants::DUPLICATE_TXID_PREFIXES};

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub db: Database,

    pub addresstype_to_addresshash_to_addressindex: ByAddressType<Store<AddressHash, TypeIndex>>,
    pub addresstype_to_addressindex_and_txindex: ByAddressType<Store<AddressIndexTxIndex, Unit>>,
    pub addresstype_to_addressindex_and_unspentoutpoint:
        ByAddressType<Store<AddressIndexOutPoint, Unit>>,
    pub blockhashprefix_to_height: Store<BlockHashPrefix, Height>,
    pub height_to_coinbase_tag: Store<Height, StoredString>,
    pub txidprefix_to_txindex: Store<TxidPrefix, TxIndex>,
}

impl Stores {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let pathbuf = parent.join("stores");
        let path = pathbuf.as_path();

        fs::create_dir_all(&pathbuf)?;

        let database = match brk_store::open_database(path) {
            Ok(database) => database,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(parent, version);
            }
        };

        let database_ref = &database;

        let create_addresshash_to_addressindex_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("h2i{}", index),
                version,
                Mode::PushOnly,
                Kind::Random,
            )
        };

        let create_addressindex_to_txindex_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("a2t{}", index),
                version,
                Mode::PushOnly,
                Kind::Vec,
            )
        };

        let create_addressindex_to_unspentoutpoint_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("a2u{}", index),
                version,
                Mode::Any,
                Kind::Vec,
            )
        };

        Ok(Self {
            db: database.clone(),

            height_to_coinbase_tag: Store::import(
                database_ref,
                path,
                "height_to_coinbase_tag",
                version,
                Mode::PushOnly,
                Kind::Sequential,
            )?,
            addresstype_to_addresshash_to_addressindex: ByAddressType::new_with_index(
                create_addresshash_to_addressindex_store,
            )?,
            addresstype_to_addressindex_and_txindex: ByAddressType::new_with_index(
                create_addressindex_to_txindex_store,
            )?,
            addresstype_to_addressindex_and_unspentoutpoint: ByAddressType::new_with_index(
                create_addressindex_to_unspentoutpoint_store,
            )?,
            blockhashprefix_to_height: Store::import(
                database_ref,
                path,
                "blockhashprefix_to_height",
                version,
                Mode::PushOnly,
                Kind::Random,
            )?,
            txidprefix_to_txindex: Store::import_cached(
                database_ref,
                path,
                "txidprefix_to_txindex",
                version,
                Mode::PushOnly,
                Kind::Recent,
                5,
            )?,
        })
    }

    pub fn starting_height(&self) -> Height {
        self.iter_any()
            .map(|store| store.height().map(Height::incremented).unwrap_or_default())
            .min()
            .unwrap()
    }

    fn iter_any(&self) -> impl Iterator<Item = &dyn AnyStore> {
        [
            &self.blockhashprefix_to_height as &dyn AnyStore,
            &self.height_to_coinbase_tag,
            &self.txidprefix_to_txindex,
        ]
        .into_iter()
        .chain(
            self.addresstype_to_addresshash_to_addressindex
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .values()
                .map(|s| s as &dyn AnyStore),
        )
    }

    fn par_iter_any_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStore> {
        [
            &mut self.blockhashprefix_to_height as &mut dyn AnyStore,
            &mut self.height_to_coinbase_tag,
            &mut self.txidprefix_to_txindex,
        ]
        .into_par_iter()
        .chain(
            self.addresstype_to_addresshash_to_addressindex
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        let i = Instant::now();
        self.par_iter_any_mut()
            .try_for_each(|store| store.commit(height))?;
        info!("Stores committed in {:?}", i.elapsed());

        let i = Instant::now();
        self.db.persist(PersistMode::SyncData)?;
        info!("Stores persisted in {:?}", i.elapsed());

        Ok(())
    }

    pub fn rollback_if_needed(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> Result<()> {
        if self.blockhashprefix_to_height.is_empty()?
            && self.txidprefix_to_txindex.is_empty()?
            && self.height_to_coinbase_tag.is_empty()?
            && self
                .addresstype_to_addresshash_to_addressindex
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_txindex
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_unspentoutpoint
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
        {
            return Ok(());
        }

        if starting_indexes.height != Height::ZERO {
            vecs.blocks
                .blockhash
                .iter()?
                .skip(starting_indexes.height.to_usize())
                .map(BlockHashPrefix::from)
                .for_each(|prefix| {
                    self.blockhashprefix_to_height.remove(prefix);
                });

            (starting_indexes.height.to_usize()..vecs.blocks.blockhash.len())
                .map(Height::from)
                .for_each(|h| {
                    self.height_to_coinbase_tag.remove(h);
                });

            // Remove address hashes for all address types starting from rollback height
            // (each address only appears once in bytes vec, so no dedup needed)
            for address_type in [
                OutputType::P2PK65,
                OutputType::P2PK33,
                OutputType::P2PKH,
                OutputType::P2SH,
                OutputType::P2WPKH,
                OutputType::P2WSH,
                OutputType::P2TR,
                OutputType::P2A,
            ] {
                for hash in vecs.iter_address_hashes_from(address_type, starting_indexes.height)? {
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(address_type)
                        .remove(hash);
                }
            }
        } else {
            unreachable!();
        }

        if starting_indexes.txindex != TxIndex::ZERO {
            vecs.transactions
                .txid
                .iter()?
                .enumerate()
                .skip(starting_indexes.txindex.to_usize())
                .for_each(|(txindex, txid)| {
                    let txindex = TxIndex::from(txindex);
                    let txidprefix = TxidPrefix::from(&txid);

                    let is_known_dup =
                        DUPLICATE_TXID_PREFIXES
                            .iter()
                            .any(|(dup_prefix, dup_txindex)| {
                                txindex == *dup_txindex && txidprefix == *dup_prefix
                            });

                    if !is_known_dup {
                        self.txidprefix_to_txindex.remove(txidprefix);
                    }
                });
        } else {
            unreachable!();
        }

        if starting_indexes.txoutindex != TxOutIndex::ZERO {
            let mut txoutindex_to_txindex_iter = vecs.outputs.txindex.iter()?;
            let mut txindex_to_first_txoutindex_iter =
                vecs.transactions.first_txoutindex.iter()?;
            let mut txoutindex_to_outputtype_iter = vecs.outputs.outputtype.iter()?;
            let mut txoutindex_to_typeindex_iter = vecs.outputs.typeindex.iter()?;

            // Collect unique (addresstype, addressindex, txindex) to avoid double deletion
            // when same address receives multiple outputs in same transaction
            let mut addressindex_txindex_to_remove: FxHashSet<(OutputType, TypeIndex, TxIndex)> =
                FxHashSet::default();

            for txoutindex in
                starting_indexes.txoutindex.to_usize()..vecs.outputs.outputtype.len()
            {
                let outputtype = txoutindex_to_outputtype_iter.get_at_unwrap(txoutindex);
                if !outputtype.is_address() {
                    continue;
                }

                let addresstype = outputtype;
                let addressindex = txoutindex_to_typeindex_iter.get_at_unwrap(txoutindex);
                let txindex = txoutindex_to_txindex_iter.get_at_unwrap(txoutindex);

                addressindex_txindex_to_remove.insert((addresstype, addressindex, txindex));

                let vout = Vout::from(
                    txoutindex
                        - txindex_to_first_txoutindex_iter
                            .get_unwrap(txindex)
                            .to_usize(),
                );
                let outpoint = OutPoint::new(txindex, vout);

                // OutPoints are unique per output, no dedup needed
                self.addresstype_to_addressindex_and_unspentoutpoint
                    .get_mut_unwrap(addresstype)
                    .remove(AddressIndexOutPoint::from((addressindex, outpoint)));
            }

            // Don't remove yet - merge with second loop's set first

            // Collect outputs that were spent after the rollback point
            // We need to: 1) reset their spend status, 2) restore address stores
            let mut txindex_to_first_txoutindex_iter =
                vecs.transactions.first_txoutindex.iter()?;
            let mut txoutindex_to_outputtype_iter = vecs.outputs.outputtype.iter()?;
            let mut txoutindex_to_typeindex_iter = vecs.outputs.typeindex.iter()?;
            let mut txinindex_to_txindex_iter = vecs.inputs.txindex.iter()?;

            let outputs_to_unspend: Vec<_> = vecs
                .inputs
                .outpoint
                .iter()?
                .enumerate()
                .skip(starting_indexes.txinindex.to_usize())
                .filter_map(|(txinindex, outpoint): (usize, OutPoint)| {
                    if outpoint.is_coinbase() {
                        return None;
                    }

                    let output_txindex = outpoint.txindex();
                    let vout = outpoint.vout();

                    // Calculate txoutindex from output's txindex and vout
                    let txoutindex =
                        txindex_to_first_txoutindex_iter.get_unwrap(output_txindex) + vout;

                    // Only process if this output was created before the rollback point
                    if txoutindex < starting_indexes.txoutindex {
                        let outputtype = txoutindex_to_outputtype_iter.get_unwrap(txoutindex);
                        let typeindex = txoutindex_to_typeindex_iter.get_unwrap(txoutindex);
                        let spending_txindex = txinindex_to_txindex_iter.get_at_unwrap(txinindex);

                        Some((outpoint, outputtype, typeindex, spending_txindex))
                    } else {
                        None
                    }
                })
                .collect();

            // Now process the collected outputs (iterators dropped, can mutate vecs)
            // Add spending tx entries to the same set (avoid double deletion when same tx
            // both creates output to address A and spends output from address A)
            for (outpoint, outputtype, typeindex, spending_txindex) in outputs_to_unspend {
                // Restore address stores if this is an address output
                if outputtype.is_address() {
                    let addresstype = outputtype;
                    let addressindex = typeindex;

                    // Add to same set as first loop
                    addressindex_txindex_to_remove.insert((addresstype, addressindex, spending_txindex));

                    // OutPoints are unique, no dedup needed for insert
                    self.addresstype_to_addressindex_and_unspentoutpoint
                        .get_mut_unwrap(addresstype)
                        .insert(AddressIndexOutPoint::from((addressindex, outpoint)), Unit);
                }
            }

            // Now remove all deduplicated addressindex_txindex entries (from both loops)
            for (addresstype, addressindex, txindex) in addressindex_txindex_to_remove {
                self.addresstype_to_addressindex_and_txindex
                    .get_mut_unwrap(addresstype)
                    .remove(AddressIndexTxIndex::from((addressindex, txindex)));
            }
        } else {
            unreachable!();
        }

        // Force-lower the height on all stores before committing.
        // This is necessary because commit() only updates the height if needed,
        // but during rollback we must lower it even if it's already higher.
        let rollback_height = starting_indexes.height.decremented().unwrap_or_default();
        self.par_iter_any_mut()
            .try_for_each(|store| store.export_meta(rollback_height))?;

        self.commit(rollback_height)?;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        info!("Resetting stores...");

        // Clear all stores (both in-memory buffers and on-disk keyspaces)
        self.par_iter_any_mut()
            .try_for_each(|store| store.reset())?;

        // Persist the cleared state
        self.db.persist(PersistMode::SyncAll)?;

        Ok(())
    }
}
