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
use vecdb::{AnyVec, ReadableVec, VecIndex};

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
        Self::forced_import_inner(parent, version, true)
    }

    fn forced_import_inner(parent: &Path, version: Version, can_retry: bool) -> Result<Self> {
        let pathbuf = parent.join("stores");
        let path = pathbuf.as_path();

        fs::create_dir_all(&pathbuf)?;

        let database = match brk_store::open_database(path) {
            Ok(database) => database,
            Err(_) if can_retry => {
                fs::remove_dir_all(path)?;
                return Self::forced_import_inner(parent, version, false);
            }
            Err(err) => return Err(err.into()),
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
        if self.is_empty()? {
            return Ok(());
        }

        debug_assert!(starting_indexes.height != Height::ZERO);
        debug_assert!(starting_indexes.txindex != TxIndex::ZERO);
        debug_assert!(starting_indexes.txoutindex != TxOutIndex::ZERO);

        self.rollback_block_metadata(vecs, starting_indexes)?;
        self.rollback_txids(vecs, starting_indexes);
        self.rollback_outputs_and_inputs(vecs, starting_indexes);

        let rollback_height = starting_indexes.height.decremented().unwrap_or_default();
        self.par_iter_any_mut()
            .try_for_each(|store| store.export_meta(rollback_height))?;
        self.commit(rollback_height)?;

        Ok(())
    }

    fn is_empty(&self) -> Result<bool> {
        Ok(self.blockhashprefix_to_height.is_empty()?
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
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?)
    }

    fn rollback_block_metadata(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> Result<()> {
        vecs.blocks.blockhash.for_each_range_at(
            starting_indexes.height.to_usize(),
            vecs.blocks.blockhash.len(),
            |blockhash| {
                self.blockhashprefix_to_height
                    .remove(BlockHashPrefix::from(blockhash));
            },
        );

        (starting_indexes.height.to_usize()..vecs.blocks.blockhash.len())
            .map(Height::from)
            .for_each(|h| {
                self.height_to_coinbase_tag.remove(h);
            });

        for address_type in OutputType::ADDRESS_TYPES {
            for hash in vecs.iter_address_hashes_from(address_type, starting_indexes.height)? {
                self.addresstype_to_addresshash_to_addressindex
                    .get_mut_unwrap(address_type)
                    .remove(hash);
            }
        }

        Ok(())
    }

    fn rollback_txids(&mut self, vecs: &mut Vecs, starting_indexes: &Indexes) {
        let start = starting_indexes.txindex.to_usize();
        let end = vecs.transactions.txid.len();
        let mut current_index = start;
        vecs.transactions.txid.for_each_range_at(start, end, |txid| {
            let txindex = TxIndex::from(current_index);
            let txidprefix = TxidPrefix::from(&txid);

            let is_known_dup = DUPLICATE_TXID_PREFIXES
                .iter()
                .any(|(dup_prefix, dup_txindex)| {
                    txindex == *dup_txindex && txidprefix == *dup_prefix
                });

            if !is_known_dup {
                self.txidprefix_to_txindex.remove(txidprefix);
            }
            current_index += 1;
        });

        self.txidprefix_to_txindex.clear_caches();
    }

    fn rollback_outputs_and_inputs(&mut self, vecs: &mut Vecs, starting_indexes: &Indexes) {
        let txindex_to_first_txoutindex_reader = vecs.transactions.first_txoutindex.reader();
        let txoutindex_to_outputtype_reader = vecs.outputs.outputtype.reader();
        let txoutindex_to_typeindex_reader = vecs.outputs.typeindex.reader();

        let mut addressindex_txindex_to_remove: FxHashSet<(OutputType, TypeIndex, TxIndex)> =
            FxHashSet::default();

        let rollback_start = starting_indexes.txoutindex.to_usize();
        let rollback_end = vecs.outputs.outputtype.len();

        let txindexes: Vec<TxIndex> =
            vecs.outputs.txindex.collect_range_at(rollback_start, rollback_end);

        for (i, txoutindex) in (rollback_start..rollback_end).enumerate() {
            let outputtype = txoutindex_to_outputtype_reader.get(txoutindex);
            if !outputtype.is_address() {
                continue;
            }

            let addresstype = outputtype;
            let addressindex = txoutindex_to_typeindex_reader.get(txoutindex);
            let txindex = txindexes[i];

            addressindex_txindex_to_remove.insert((addresstype, addressindex, txindex));

            let vout = Vout::from(
                txoutindex
                    - txindex_to_first_txoutindex_reader
                        .get(txindex.to_usize())
                        .to_usize(),
            );
            let outpoint = OutPoint::new(txindex, vout);

            self.addresstype_to_addressindex_and_unspentoutpoint
                .get_mut_unwrap(addresstype)
                .remove(AddressIndexOutPoint::from((addressindex, outpoint)));
        }

        let start = starting_indexes.txinindex.to_usize();
        let end = vecs.inputs.outpoint.len();
        let outpoints: Vec<OutPoint> = vecs.inputs.outpoint.collect_range_at(start, end);
        let spending_txindexes: Vec<TxIndex> = vecs.inputs.txindex.collect_range_at(start, end);

        let outputs_to_unspend: Vec<_> = outpoints
            .into_iter()
            .zip(spending_txindexes)
            .filter_map(|(outpoint, spending_txindex)| {
                if outpoint.is_coinbase() {
                    return None;
                }

                let output_txindex = outpoint.txindex();
                let vout = outpoint.vout();
                let txoutindex =
                    txindex_to_first_txoutindex_reader.get(output_txindex.to_usize()) + vout;

                if txoutindex < starting_indexes.txoutindex {
                    let outputtype = txoutindex_to_outputtype_reader.get(txoutindex.to_usize());
                    let typeindex = txoutindex_to_typeindex_reader.get(txoutindex.to_usize());
                    Some((outpoint, outputtype, typeindex, spending_txindex))
                } else {
                    None
                }
            })
            .collect();

        for (outpoint, outputtype, typeindex, spending_txindex) in outputs_to_unspend {
            if outputtype.is_address() {
                let addresstype = outputtype;
                let addressindex = typeindex;

                addressindex_txindex_to_remove.insert((
                    addresstype,
                    addressindex,
                    spending_txindex,
                ));

                self.addresstype_to_addressindex_and_unspentoutpoint
                    .get_mut_unwrap(addresstype)
                    .insert(AddressIndexOutPoint::from((addressindex, outpoint)), Unit);
            }
        }

        for (addresstype, addressindex, txindex) in addressindex_txindex_to_remove {
            self.addresstype_to_addressindex_and_txindex
                .get_mut_unwrap(addresstype)
                .remove(AddressIndexTxIndex::from((addressindex, txindex)));
        }
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
