use std::{fs, path::Path, time::Instant};

use rustc_hash::FxHashSet;

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_store::{AnyStore, Kind, Mode, Store};
use brk_types::{
    AddrHash, AddrIndexOutPoint, AddrIndexTxIndex, BlockHashPrefix, Height, OutPoint,
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

    pub addr_type_to_addr_hash_to_addr_index: ByAddrType<Store<AddrHash, TypeIndex>>,
    pub addr_type_to_addr_index_and_tx_index: ByAddrType<Store<AddrIndexTxIndex, Unit>>,
    pub addr_type_to_addr_index_and_unspent_outpoint:
        ByAddrType<Store<AddrIndexOutPoint, Unit>>,
    pub blockhash_prefix_to_height: Store<BlockHashPrefix, Height>,
    pub height_to_coinbase_tag: Store<Height, StoredString>,
    pub txid_prefix_to_tx_index: Store<TxidPrefix, TxIndex>,
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

        let create_addr_hash_to_addr_index_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("h2i{}", index),
                version,
                Mode::PushOnly,
                Kind::Random,
            )
        };

        let create_addr_index_to_tx_index_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("a2t{}", index),
                version,
                Mode::PushOnly,
                Kind::Vec,
            )
        };

        let create_addr_index_to_unspent_outpoint_store = |index| {
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
            addr_type_to_addr_hash_to_addr_index: ByAddrType::new_with_index(
                create_addr_hash_to_addr_index_store,
            )?,
            addr_type_to_addr_index_and_tx_index: ByAddrType::new_with_index(
                create_addr_index_to_tx_index_store,
            )?,
            addr_type_to_addr_index_and_unspent_outpoint: ByAddrType::new_with_index(
                create_addr_index_to_unspent_outpoint_store,
            )?,
            blockhash_prefix_to_height: Store::import(
                database_ref,
                path,
                "blockhash_prefix_to_height",
                version,
                Mode::PushOnly,
                Kind::Random,
            )?,
            txid_prefix_to_tx_index: Store::import_cached(
                database_ref,
                path,
                "txid_prefix_to_tx_index",
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
            &self.blockhash_prefix_to_height as &dyn AnyStore,
            &self.height_to_coinbase_tag,
            &self.txid_prefix_to_tx_index,
        ]
        .into_iter()
        .chain(
            self.addr_type_to_addr_hash_to_addr_index
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addr_type_to_addr_index_and_tx_index
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addr_type_to_addr_index_and_unspent_outpoint
                .values()
                .map(|s| s as &dyn AnyStore),
        )
    }

    fn par_iter_any_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStore> {
        [
            &mut self.blockhash_prefix_to_height as &mut dyn AnyStore,
            &mut self.height_to_coinbase_tag,
            &mut self.txid_prefix_to_tx_index,
        ]
        .into_par_iter()
        .chain(
            self.addr_type_to_addr_hash_to_addr_index
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addr_type_to_addr_index_and_tx_index
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addr_type_to_addr_index_and_unspent_outpoint
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
        debug_assert!(starting_indexes.tx_index != TxIndex::ZERO);
        debug_assert!(starting_indexes.txout_index != TxOutIndex::ZERO);

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
        Ok(self.blockhash_prefix_to_height.is_empty()?
            && self.txid_prefix_to_tx_index.is_empty()?
            && self.height_to_coinbase_tag.is_empty()?
            && self
                .addr_type_to_addr_hash_to_addr_index
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addr_type_to_addr_index_and_tx_index
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addr_type_to_addr_index_and_unspent_outpoint
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
                self.blockhash_prefix_to_height
                    .remove(BlockHashPrefix::from(blockhash));
            },
        );

        (starting_indexes.height.to_usize()..vecs.blocks.blockhash.len())
            .map(Height::from)
            .for_each(|h| {
                self.height_to_coinbase_tag.remove(h);
            });

        for addr_type in OutputType::ADDR_TYPES {
            for hash in vecs.iter_addr_hashes_from(addr_type, starting_indexes.height)? {
                self.addr_type_to_addr_hash_to_addr_index
                    .get_mut_unwrap(addr_type)
                    .remove(hash);
            }
        }

        Ok(())
    }

    fn rollback_txids(&mut self, vecs: &mut Vecs, starting_indexes: &Indexes) {
        let start = starting_indexes.tx_index.to_usize();
        let end = vecs.transactions.txid.len();
        let mut current_index = start;
        vecs.transactions
            .txid
            .for_each_range_at(start, end, |txid| {
                let tx_index = TxIndex::from(current_index);
                let txid_prefix = TxidPrefix::from(&txid);

                let is_known_dup =
                    DUPLICATE_TXID_PREFIXES
                        .iter()
                        .any(|(dup_prefix, dup_tx_index)| {
                            tx_index == *dup_tx_index && txid_prefix == *dup_prefix
                        });

                if !is_known_dup {
                    self.txid_prefix_to_tx_index.remove(txid_prefix);
                }
                current_index += 1;
            });

        self.txid_prefix_to_tx_index.clear_caches();
    }

    fn rollback_outputs_and_inputs(&mut self, vecs: &mut Vecs, starting_indexes: &Indexes) {
        let tx_index_to_first_txout_index_reader = vecs.transactions.first_txout_index.reader();
        let txout_index_to_output_type_reader = vecs.outputs.output_type.reader();
        let txout_index_to_type_index_reader = vecs.outputs.type_index.reader();

        let mut addr_index_tx_index_to_remove: FxHashSet<(OutputType, TypeIndex, TxIndex)> =
            FxHashSet::default();

        let rollback_start = starting_indexes.txout_index.to_usize();
        let rollback_end = vecs.outputs.output_type.len();

        let tx_indexes: Vec<TxIndex> = vecs
            .outputs
            .tx_index
            .collect_range_at(rollback_start, rollback_end);

        for (i, txout_index) in (rollback_start..rollback_end).enumerate() {
            let output_type = txout_index_to_output_type_reader.get(txout_index);
            if !output_type.is_addr() {
                continue;
            }

            let addr_type = output_type;
            let addr_index = txout_index_to_type_index_reader.get(txout_index);
            let tx_index = tx_indexes[i];

            addr_index_tx_index_to_remove.insert((addr_type, addr_index, tx_index));

            let vout = Vout::from(
                txout_index
                    - tx_index_to_first_txout_index_reader
                        .get(tx_index.to_usize())
                        .to_usize(),
            );
            let outpoint = OutPoint::new(tx_index, vout);

            self.addr_type_to_addr_index_and_unspent_outpoint
                .get_mut_unwrap(addr_type)
                .remove(AddrIndexOutPoint::from((addr_index, outpoint)));
        }

        let start = starting_indexes.txin_index.to_usize();
        let end = vecs.inputs.outpoint.len();
        let outpoints: Vec<OutPoint> = vecs.inputs.outpoint.collect_range_at(start, end);
        let spending_tx_indexes: Vec<TxIndex> = vecs.inputs.tx_index.collect_range_at(start, end);

        let outputs_to_unspend: Vec<_> = outpoints
            .into_iter()
            .zip(spending_tx_indexes)
            .filter_map(|(outpoint, spending_tx_index)| {
                if outpoint.is_coinbase() {
                    return None;
                }

                let output_tx_index = outpoint.tx_index();
                let vout = outpoint.vout();
                let txout_index =
                    tx_index_to_first_txout_index_reader.get(output_tx_index.to_usize()) + vout;

                if txout_index < starting_indexes.txout_index {
                    let output_type = txout_index_to_output_type_reader.get(txout_index.to_usize());
                    let type_index = txout_index_to_type_index_reader.get(txout_index.to_usize());
                    Some((outpoint, output_type, type_index, spending_tx_index))
                } else {
                    None
                }
            })
            .collect();

        for (outpoint, output_type, type_index, spending_tx_index) in outputs_to_unspend {
            if output_type.is_addr() {
                let addr_type = output_type;
                let addr_index = type_index;

                addr_index_tx_index_to_remove.insert((
                    addr_type,
                    addr_index,
                    spending_tx_index,
                ));

                self.addr_type_to_addr_index_and_unspent_outpoint
                    .get_mut_unwrap(addr_type)
                    .insert(AddrIndexOutPoint::from((addr_index, outpoint)), Unit);
            }
        }

        for (addr_type, addr_index, tx_index) in addr_index_tx_index_to_remove {
            self.addr_type_to_addr_index_and_tx_index
                .get_mut_unwrap(addr_type)
                .remove(AddrIndexTxIndex::from((addr_index, tx_index)));
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
