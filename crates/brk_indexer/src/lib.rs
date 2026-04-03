#![doc = include_str!("../README.md")]

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_reader::{Reader, XORBytes};
use brk_rpc::Client;
use brk_types::{BlockHash, Height};
use fjall::PersistMode;
use parking_lot::RwLock;
use tracing::{debug, info};
use vecdb::{
    Exit, RawDBError, ReadOnlyClone, ReadableVec, Ro, Rw, StorageMode, WritableVec, unlikely,
};
mod constants;
mod indexes;
mod processor;
mod readers;
mod stores;
mod vecs;

use constants::*;
use indexes::IndexesExt;
use processor::{BlockBuffers, BlockProcessor};
use readers::Readers;

pub use brk_types::Indexes;
pub use stores::Stores;
pub use vecs::*;

pub struct Indexer<M: StorageMode = Rw> {
    path: PathBuf,
    pub vecs: Vecs<M>,
    pub stores: Stores,
    tip_blockhash: Arc<RwLock<BlockHash>>,
}

impl<M: StorageMode> Indexer<M> {
    pub fn tip_blockhash(&self) -> BlockHash {
        self.tip_blockhash.read().clone()
    }
}

impl ReadOnlyClone for Indexer {
    type ReadOnly = Indexer<Ro>;

    fn read_only_clone(&self) -> Indexer<Ro> {
        Indexer {
            path: self.path.clone(),
            vecs: self.vecs.read_only_clone(),
            stores: self.stores.clone(),
            tip_blockhash: self.tip_blockhash.clone(),
        }
    }
}

impl Indexer {
    pub fn forced_import(outputs_dir: &Path) -> Result<Self> {
        Self::forced_import_inner(outputs_dir, true)
    }

    fn forced_import_inner(outputs_dir: &Path, can_retry: bool) -> Result<Self> {
        info!("Increasing number of open files limit...");
        let no_file_limit = rlimit::getrlimit(rlimit::Resource::NOFILE)?;
        rlimit::setrlimit(
            rlimit::Resource::NOFILE,
            no_file_limit.0.max(10_000),
            no_file_limit.1,
        )?;

        info!("Importing indexer...");

        let indexed_path = outputs_dir.join("indexed");

        let try_import = || -> Result<Self> {
            let i = Instant::now();
            let vecs = Vecs::forced_import(&indexed_path, VERSION)?;
            info!("Imported vecs in {:?}", i.elapsed());

            let i = Instant::now();
            let stores = Stores::forced_import(&indexed_path, VERSION)?;
            info!("Imported stores in {:?}", i.elapsed());

            let tip_blockhash = vecs.blocks.blockhash.collect_last().unwrap_or_default();

            Ok(Self {
                path: indexed_path.clone(),
                vecs,
                stores,
                tip_blockhash: Arc::new(RwLock::new(tip_blockhash)),
            })
        };

        match try_import() {
            Ok(result) => Ok(result),
            Err(err) if err.is_lock_error() => {
                // Lock errors are transient - another process has the database open.
                // Don't delete data, just return the error.
                Err(err)
            }
            Err(err) if can_retry && err.is_data_error() => {
                // Data corruption or version mismatch - safe to delete and retry
                info!("{err:?}, deleting {indexed_path:?} and retrying");
                fs::remove_dir_all(&indexed_path)?;
                Self::forced_import_inner(outputs_dir, false)
            }
            Err(err) => Err(err),
        }
    }

    /// Fully resets the indexer by deleting stores from disk and reimporting.
    /// Unlike stores.reset() which uses keyspace.clear() (leaving a journal
    /// record that gets replayed on every recovery), this cleanly recreates.
    fn full_reset(&mut self) -> Result<()> {
        info!("Full reset...");
        self.vecs.reset()?;
        let stores_path = self.path.join("stores");
        fs::remove_dir_all(&stores_path).ok();
        self.stores = Stores::forced_import(&self.path, VERSION)?;
        Ok(())
    }

    pub fn index(&mut self, reader: &Reader, client: &Client, exit: &Exit) -> Result<Indexes> {
        self.index_(reader, client, exit, false)
    }

    pub fn checked_index(
        &mut self,
        reader: &Reader,
        client: &Client,
        exit: &Exit,
    ) -> Result<Indexes> {
        self.index_(reader, client, exit, true)
    }

    fn check_xor_bytes(&mut self, reader: &Reader) -> Result<()> {
        let current = reader.xor_bytes();
        let cached = XORBytes::from(self.path.as_path());

        if cached == current {
            return Ok(());
        }

        self.full_reset()?;

        fs::write(self.path.join("xor.dat"), *current)?;

        Ok(())
    }

    fn index_(
        &mut self,
        reader: &Reader,
        client: &Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<Indexes> {
        self.vecs.db.sync_bg_tasks()?;

        self.check_xor_bytes(reader)?;

        debug!("Starting indexing...");

        let last_blockhash = self.vecs.blocks.blockhash.collect_last();
        // Rollback sim
        // let last_blockhash = self
        //     .vecs
        //     .blocks
        //     .blockhash
        //     .collect_one_at(self.vecs.blocks.blockhash.len() - 2);
        debug!("Last block hash found.");

        let (starting_indexes, prev_hash) = if let Some(hash) = last_blockhash {
            let (height, hash) = client.get_closest_valid_height(hash)?;
            match Indexes::from_vecs_and_stores(height.incremented(), &mut self.vecs, &self.stores)
            {
                Some(starting_indexes) => {
                    if starting_indexes.height > client.get_last_height()? {
                        info!("Up to date, nothing to index.");
                        return Ok(starting_indexes);
                    }
                    (starting_indexes, Some(hash))
                }
                None => {
                    info!("Data inconsistency detected, resetting indexer...");
                    self.full_reset()?;
                    (Indexes::default(), None)
                }
            }
        } else {
            (Indexes::default(), None)
        };
        debug!("Starting indexes set.");

        let lock = exit.lock();
        self.stores
            .rollback_if_needed(&mut self.vecs, &starting_indexes)?;
        debug!("Rollback stores done.");
        self.vecs.rollback_if_needed(&starting_indexes)?;
        debug!("Rollback vecs done.");
        drop(lock);

        // Cloned because we want to return starting indexes for the computer
        let mut indexes = starting_indexes.clone();
        debug!("Indexes cloned.");

        let is_export_height =
            |height: Height| -> bool { height != 0 && height % SNAPSHOT_BLOCK_RANGE == 0 };

        let export = move |stores: &mut Stores, vecs: &mut Vecs, height: Height| -> Result<()> {
            info!("Exporting...");
            let i = Instant::now();
            let _lock = exit.lock();
            thread::scope(|s| -> Result<()> {
                let stores_res = s.spawn(|| -> Result<()> {
                    let i = Instant::now();
                    stores.commit(height)?;
                    debug!("Stores exported in {:?}", i.elapsed());
                    Ok(())
                });
                let vecs_res = s.spawn(|| -> Result<()> {
                    let i = Instant::now();
                    vecs.flush(height)?;
                    debug!("Vecs exported in {:?}", i.elapsed());
                    Ok(())
                });
                stores_res.join().unwrap()?;
                vecs_res.join().unwrap()?;
                Ok(())
            })?;
            info!("Exported in {:?}", i.elapsed());
            Ok(())
        };

        let mut readers = Readers::new(&self.vecs);
        let mut buffers = BlockBuffers::default();

        let vecs = &mut self.vecs;
        let stores = &mut self.stores;

        for block in reader.after(prev_hash)?.iter() {
            let height = block.height();

            if unlikely(height.is_multiple_of(100)) {
                info!("Indexing block {height}...");
            } else {
                debug!("Indexing block {height}...");
            }

            indexes.height = height;

            vecs.blocks.position.push(block.metadata().position());
            block.tx_metadata().iter().for_each(|m| {
                vecs.transactions.position.push(m.position());
            });

            let mut processor = BlockProcessor {
                block: &block,
                height,
                check_collisions,
                indexes: &mut indexes,
                vecs,
                stores,
                readers: &readers,
            };

            processor.process_block_metadata()?;

            let txs = processor.compute_txids()?;

            processor.push_block_size_and_weight(&txs)?;

            let (txins_result, txouts_result) = rayon::join(
                || processor.process_inputs(&txs, &mut buffers.txid_prefix_map),
                || processor.process_outputs(),
            );
            let txins = txins_result?;
            let txouts = txouts_result?;

            let tx_count = block.txdata.len();
            let input_count = txins.len();
            let output_count = txouts.len();

            BlockProcessor::collect_same_block_spent_outpoints(
                &txins,
                &mut buffers.same_block_spent,
            );

            processor.check_txid_collisions(&txs)?;

            processor.finalize_and_store_metadata(
                txs,
                txouts,
                txins,
                &buffers.same_block_spent,
                &mut buffers.already_added_addrs,
                &mut buffers.same_block_output_info,
            )?;

            processor.update_indexes(tx_count, input_count, output_count);

            if is_export_height(height) {
                drop(readers);
                export(stores, vecs, height)?;
                readers = Readers::new(vecs);
            }

            *self.tip_blockhash.write() = block.block_hash().into();
        }

        drop(readers);

        let lock = exit.lock();
        let tasks = self.stores.take_all_pending_ingests(indexes.height)?;
        self.vecs.stamped_write(indexes.height)?;
        let fjall_db = self.stores.db.clone();

        self.vecs.db.run_bg(move |db| {
            let _lock = lock;

            sleep(Duration::from_secs(5));

            info!("Exporting...");
            let i = Instant::now();

            if !tasks.is_empty() {
                let i = Instant::now();
                for task in tasks {
                    task().map_err(vecdb::RawDBError::other)?;
                }
                debug!("Stores committed in {:?}", i.elapsed());

                let i = Instant::now();
                fjall_db
                    .persist(PersistMode::SyncData)
                    .map_err(RawDBError::other)?;
                debug!("Stores persisted in {:?}", i.elapsed());
            }

            db.compact()?;

            info!("Exported in {:?}", i.elapsed());
            Ok(())
        });

        Ok(starting_indexes)
    }
}
