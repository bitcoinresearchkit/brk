#![doc = include_str!("../README.md")]

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_reader::{Reader, XORBytes};
use brk_rpc::Client;
use brk_types::{BlockHash, Height};
use fjall::PersistMode;
use parking_lot::RwLock;
use tracing::{debug, error, info};
use vecdb::{
    Exit, RawDBError, ReadOnlyClone, ReadableVec, Ro, Rw, StorageMode, WritableVec, unlikely,
};
mod constants;
mod lengths;
mod processor;
mod readers;
mod safe_lengths;
mod stores;
mod vecs;

use constants::*;
use processor::{BlockBuffers, BlockProcessor};
use readers::Readers;

pub use lengths::Lengths;
pub use safe_lengths::SafeLengths;
pub use stores::Stores;
pub use vecs::*;

pub struct Indexer<M: StorageMode = Rw> {
    path: PathBuf,
    pub vecs: Vecs<M>,
    pub stores: Stores,
    tip_blockhash: Arc<RwLock<BlockHash>>,
    safe_lengths: SafeLengths,
}

impl<M: StorageMode> Indexer<M> {
    pub fn tip_blockhash(&self) -> BlockHash {
        *self.tip_blockhash.read()
    }

    /// Pipeline-safe `Lengths` snapshot shared with `Query`. Writers
    /// advance and lower this internally; readers clamp non-series
    /// answers against this loaded snapshot.
    pub fn safe_lengths(&self) -> Lengths {
        self.safe_lengths.load()
    }
}

impl Indexer<Ro> {
    /// Live indexer stamp for diagnostics. For data reads use
    /// [`crate::SafeLengths::load`] (via `Query::height`).
    pub fn indexed_height(&self) -> Height {
        Height::from(self.vecs.blocks.blockhash.inner.stamp())
    }
}

impl Indexer {
    pub fn forced_import(outputs_dir: &Path) -> Result<Self> {
        Self::forced_import_inner(outputs_dir, true)
    }

    fn forced_import_inner(outputs_dir: &Path, can_retry: bool) -> Result<Self> {
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

            let safe_lengths = SafeLengths::new();
            if let Some(lengths) = Lengths::from_local(&vecs, &stores) {
                safe_lengths.advance(lengths);
            }

            Ok(Self {
                path: indexed_path.clone(),
                vecs,
                stores,
                tip_blockhash: Arc::new(RwLock::new(tip_blockhash)),
                safe_lengths,
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
        self.safe_lengths.reset();
        *self.tip_blockhash.write() = BlockHash::default();
        self.vecs.reset()?;
        let stores_path = self.path.join("stores");
        fs::remove_dir_all(&stores_path).ok();
        self.stores = Stores::forced_import(&self.path, VERSION)?;
        Ok(())
    }

    pub fn index(&mut self, reader: &Reader, client: &Client, exit: &Exit) -> Result<()> {
        self.index_(reader, client, exit, false)
    }

    pub fn checked_index(&mut self, reader: &Reader, client: &Client, exit: &Exit) -> Result<()> {
        self.index_(reader, client, exit, true)
    }

    fn index_(
        &mut self,
        reader: &Reader,
        client: &Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<()> {
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

        let (starting_lengths, prev_hash) = if let Some(hash) = last_blockhash {
            let (height, hash) = client.get_closest_valid_height(hash)?;
            match Lengths::resume_at(height.incremented(), &self.vecs, &self.stores) {
                Some(starting_lengths) => {
                    if starting_lengths.height > client.get_last_height()? {
                        info!("Up to date, nothing to index.");
                        return Ok(());
                    }
                    (starting_lengths, Some(hash))
                }
                None => {
                    info!("Data inconsistency detected, resetting indexer...");
                    self.full_reset()?;
                    (Lengths::default(), None)
                }
            }
        } else {
            (Lengths::default(), None)
        };
        debug!("Starting lengths set.");

        let lock = exit.lock();
        self.safe_lengths.lower_before(&starting_lengths);
        self.stores
            .rollback_if_needed(&mut self.vecs, &starting_lengths)?;
        debug!("Rollback stores done.");
        self.vecs.rollback_if_needed(&starting_lengths)?;
        debug!("Rollback vecs done.");
        if let Some(hash) = prev_hash.as_ref() {
            *self.tip_blockhash.write() = *hash;
        }
        drop(lock);

        let mut lengths = starting_lengths;

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
            let block = match block {
                Ok(block) => block,
                Err(e) => {
                    // The reader hit an unrecoverable mid-stream issue
                    // (chain break, parse failure, missing blocks).
                    // Stop cleanly so what we've already indexed gets
                    // flushed in the post-loop export — the next
                    // `index` call will resume from the new tip.
                    error!("Reader stream stopped early: {e}");
                    break;
                }
            };
            let height = block.height();

            if unlikely(height.is_multiple_of(100)) {
                info!("Indexing block {height}...");
            } else {
                debug!("Indexing block {height}...");
            }

            lengths.height = height;

            vecs.blocks.position.push(block.metadata().position());
            block.tx_metadata().iter().for_each(|m| {
                vecs.transactions.position.push(m.position());
            });

            let mut processor = BlockProcessor {
                block: &block,
                height,
                check_collisions,
                lengths: &mut lengths,
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

            let sigops = processor.compute_sigops(&txins, &txouts);

            processor.finalize_and_store_metadata(
                txs,
                txouts,
                txins,
                sigops,
                &buffers.same_block_spent,
                &mut buffers.already_added_addrs,
                &mut buffers.same_block_output_info,
            )?;

            processor
                .lengths
                .add_block(tx_count, input_count, output_count);

            if is_export_height(height) {
                drop(readers);
                export(stores, vecs, height)?;
                readers = Readers::new(vecs);
            }

            *self.tip_blockhash.write() = block.block_hash().into();
        }

        drop(readers);

        let lock = exit.lock();
        let tasks = self.stores.take_all_pending_ingests(lengths.height)?;
        self.vecs.stamped_write(lengths.height)?;
        let fjall_db = self.stores.db.clone();

        self.vecs.db.run_bg(move |db| {
            let _lock = lock;

            db.bg_sleep(Duration::from_secs(3));

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

        Ok(())
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

    /// Publish disk state as the new safe-lengths snapshot. Drains pending
    /// bg ingest first so stores are queryable at the new bound.
    pub fn advance_safe_lengths(&mut self) -> Result<()> {
        self.vecs.db.sync_bg_tasks()?;
        if let Some(lengths) = Lengths::from_local(&self.vecs, &self.stores) {
            self.safe_lengths.advance(lengths);
        }
        Ok(())
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
            safe_lengths: self.safe_lengths.clone(),
        }
    }
}
