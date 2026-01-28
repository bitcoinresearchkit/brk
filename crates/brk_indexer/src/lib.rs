#![doc = include_str!("../README.md")]

use std::{fs, path::Path, thread, time::Instant};

use brk_error::Result;
use brk_iterator::Blocks;
use brk_rpc::Client;
use brk_types::Height;
use tracing::{debug, info};
use vecdb::{Exit, TypedVecIterator};
mod constants;
mod indexes;
mod processor;
mod readers;
mod stores;
mod vecs;

use constants::*;
pub use indexes::*;
pub use processor::*;
pub use readers::*;
pub use stores::*;
pub use vecs::*;

#[derive(Clone)]
pub struct Indexer {
    pub vecs: Vecs,
    pub stores: Stores,
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

            Ok(Self { vecs, stores })
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

    pub fn index(&mut self, blocks: &Blocks, client: &Client, exit: &Exit) -> Result<Indexes> {
        self.index_(blocks, client, exit, false)
    }

    pub fn checked_index(
        &mut self,
        blocks: &Blocks,
        client: &Client,
        exit: &Exit,
    ) -> Result<Indexes> {
        self.index_(blocks, client, exit, true)
    }

    fn index_(
        &mut self,
        blocks: &Blocks,
        client: &Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<Indexes> {
        debug!("Starting indexing...");

        let last_blockhash = self.vecs.blocks.blockhash.iter()?.last();
        debug!("Last block hash found.");

        let (starting_indexes, prev_hash) = if let Some(hash) = last_blockhash {
            let (height, hash) = client.get_closest_valid_height(hash)?;
            // TEST: force rollback 5 blocks (only if we have enough blocks)
            let (height, hash) = if *height > 10 {
                let height = Height::from(height.checked_sub(1).unwrap());
                let hash = self.vecs.blocks.blockhash.iter()?.get(height).unwrap();
                (height, hash)
            } else {
                (height, hash)
            };
            // END TEST
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
                    // Data inconsistency detected - reset and start fresh
                    info!("Data inconsistency detected, resetting indexer...");
                    self.vecs.reset()?;
                    self.stores.reset()?;
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

        let should_export = |height: Height, rem: bool| -> bool {
            height != 0 && (height % SNAPSHOT_BLOCK_RANGE == 0) != rem
        };

        let export = move |stores: &mut Stores, vecs: &mut Vecs, height: Height| -> Result<()> {
            info!("Exporting...");
            let i = Instant::now();
            let _lock = exit.lock();
            thread::scope(|s| -> Result<()> {
                let stores_res = s.spawn(|| -> Result<()> {
                    let i = Instant::now();
                    stores.commit(height)?;
                    info!("Stores exported in {:?}", i.elapsed());
                    Ok(())
                });
                let vecs_res = s.spawn(|| -> Result<()> {
                    let i = Instant::now();
                    vecs.flush(height)?;
                    info!("Vecs exported in {:?}", i.elapsed());
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

        let vecs = &mut self.vecs;
        let stores = &mut self.stores;

        for block in blocks.after(prev_hash)? {
            let height = block.height();

            info!("Indexing block {height}...");

            indexes.height = height;

            let mut processor = BlockProcessor {
                block: &block,
                height,
                check_collisions,
                indexes: &mut indexes,
                vecs,
                stores,
                readers: &readers,
            };

            // Phase 1: Process block metadata
            processor.process_block_metadata()?;

            // Phase 2: Compute TXIDs in parallel
            let txs = processor.compute_txids()?;

            // Phase 3: Process inputs in parallel
            let txins = processor.process_inputs(&txs)?;

            // Phase 4: Collect same-block spent outpoints
            let same_block_spent_outpoints =
                BlockProcessor::collect_same_block_spent_outpoints(&txins);

            // Phase 5: Process outputs in parallel
            let txouts = processor.process_outputs()?;

            let tx_len = block.txdata.len();
            let inputs_len = txins.len();
            let outputs_len = txouts.len();

            // Phase 6: Finalize outputs sequentially
            let same_block_output_info =
                processor.finalize_outputs(txouts, &same_block_spent_outpoints)?;

            // Phase 7: Finalize inputs sequentially
            processor.finalize_inputs(txins, same_block_output_info)?;

            // Phase 8: Check TXID collisions
            processor.check_txid_collisions(&txs)?;

            // Phase 9: Store transaction metadata
            processor.store_transaction_metadata(txs)?;

            // Phase 10: Update indexes
            processor.update_indexes(tx_len, inputs_len, outputs_len);

            if should_export(height, false) {
                drop(readers);
                export(stores, vecs, height)?;
                readers = Readers::new(vecs);
            }
        }

        drop(readers);

        if should_export(indexes.height, true) {
            export(stores, vecs, indexes.height)?;
        }

        self.vecs.compact()?;

        Ok(starting_indexes)
    }
}
