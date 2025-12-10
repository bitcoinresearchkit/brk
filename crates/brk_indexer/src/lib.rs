#![doc = include_str!("../README.md")]

use std::{path::Path, thread, time::Instant};

use brk_error::Result;
use brk_iterator::Blocks;
use brk_rpc::Client;
use brk_types::Height;
use log::{debug, info};
use vecdb::Exit;
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
        info!("Importing indexer...");

        let path = outputs_dir.join("indexed");

        let (vecs, stores) = thread::scope(|s| -> Result<_> {
            let vecs = s.spawn(|| -> Result<_> {
                let i = Instant::now();
                let vecs = Vecs::forced_import(&path, VERSION)?;
                info!("Imported vecs in {:?}", i.elapsed());
                Ok(vecs)
            });

            let i = Instant::now();
            let stores = Stores::forced_import(&path, VERSION)?;
            info!("Imported stores in {:?}", i.elapsed());

            Ok((vecs.join().unwrap()?, stores))
        })?;

        Ok(Self { vecs, stores })
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

        let last_blockhash = self.vecs.height_to_blockhash.iter()?.last();
        debug!("Last block hash found.");

        let (starting_indexes, prev_hash) = if let Some(hash) = last_blockhash {
            let (height, hash) = client.get_closest_valid_height(hash)?;
            let starting_indexes =
                Indexes::from((height.incremented(), &mut self.vecs, &self.stores));
            if starting_indexes.height > client.get_last_height()? {
                info!("Up to date, nothing to index.");
                return Ok(starting_indexes);
            }
            (starting_indexes, Some(hash))
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
            let _lock = exit.lock();
            let i = Instant::now();
            stores.commit(height).unwrap();
            debug!("Commited stores in {}s", i.elapsed().as_secs());
            let i = Instant::now();
            vecs.flush(height)?;
            debug!("Flushed vecs in {}s", i.elapsed().as_secs());
            Ok(())
        };

        let mut readers = Readers::new(&self.vecs);

        let vecs = &mut self.vecs;
        let stores = &mut self.stores;

        for block in blocks.after(prev_hash)? {
            let height = block.height();

            info!("Indexing block {height}...");

            indexes.height = height;

            // Used to check rapidhash collisions
            let block_check_collisions = check_collisions && height > COLLISIONS_CHECKED_UP_TO;

            let mut processor = BlockProcessor {
                block: &block,
                height,
                check_collisions: block_check_collisions,
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
            let mut same_block_output_info =
                processor.finalize_outputs(txouts, &same_block_spent_outpoints)?;

            // Phase 7: Finalize inputs sequentially
            processor.finalize_inputs(txins, &mut same_block_output_info)?;

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
