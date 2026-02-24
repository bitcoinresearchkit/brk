use std::ops::Range;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, START_HEIGHT, bin_to_cents, cents_to_bin};
use brk_types::{Cents, OutputType, Sats, TxIndex, TxOutIndex};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, StorageMode, WritableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_prices(indexer, starting_indexes, exit)?;
        self.open
            .compute_first(starting_indexes, &self.price, indexes, exit)?;
        self.high
            .compute_max(starting_indexes, &self.price, indexes, exit)?;
        self.low
            .compute_min(starting_indexes, &self.price, indexes, exit)?;
        Ok(())
    }

    fn compute_prices(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version =
            indexer.vecs.outputs.value.version() + indexer.vecs.outputs.outputtype.version();
        self.price
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        if total_heights <= START_HEIGHT {
            return Ok(());
        }

        // Reorg: truncate to starting_indexes
        let truncate_to = self.price.len().min(starting_indexes.height.to_usize());
        self.price.truncate_if_needed_at(truncate_to)?;

        if self.price.len() < START_HEIGHT {
            for line in brk_oracle::PRICES.lines().skip(self.price.len()) {
                if self.price.len() >= START_HEIGHT {
                    break;
                }
                let dollars: f64 = line.parse().unwrap_or(0.0);
                let cents = (dollars * 100.0).round() as u64;
                self.price.push(Cents::new(cents));
            }
        }

        if self.price.len() >= total_heights {
            return Ok(());
        }

        let config = Config::default();
        let committed = self.price.len();
        let prev_cents = self.price.collect_one_at(committed - 1).unwrap();
        let seed_bin = cents_to_bin(prev_cents.inner() as f64);
        let warmup = config.window_size.min(committed - START_HEIGHT);
        let mut oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Self::feed_blocks(o, indexer, (committed - warmup)..committed);
        });

        let num_new = total_heights - committed;
        info!(
            "Computing oracle prices: {} to {} ({warmup} warmup)",
            committed, total_heights
        );

        let ref_bins = Self::feed_blocks(&mut oracle, indexer, committed..total_heights);

        for (i, ref_bin) in ref_bins.into_iter().enumerate() {
            self.price.push(Cents::new(bin_to_cents(ref_bin)));

            let progress = ((i + 1) * 100 / num_new) as u8;
            if i > 0 && progress > ((i * 100 / num_new) as u8) {
                info!("Oracle price computation: {}%", progress);
            }
        }

        {
            let _lock = exit.lock();
            self.price.write()?;
        }

        info!("Oracle prices complete: {} committed", self.price.len());

        Ok(())
    }

    /// Feed a range of blocks from the indexer into an Oracle (skipping coinbase),
    /// returning per-block ref_bin values.
    fn feed_blocks<M: StorageMode>(oracle: &mut Oracle, indexer: &Indexer<M>, range: Range<usize>) -> Vec<f64> {
        let total_txs = indexer.vecs.transactions.height.len();
        let total_outputs = indexer.vecs.outputs.value.len();

        // Pre-collect height-indexed data for the range (plus one extra for next-block lookups)
        let collect_end = (range.end + 1).min(indexer.vecs.transactions.first_txindex.len());
        let first_txindexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_range_at(range.start, collect_end);

        let out_firsts: Vec<TxOutIndex> = indexer
            .vecs
            .outputs
            .first_txoutindex
            .collect_range_at(range.start, collect_end);

        let mut ref_bins = Vec::with_capacity(range.len());

        // Cursor avoids per-block PcoVec page decompression for
        // the tx-indexed first_txoutindex lookup.  The accessed
        // txindex values (first_txindex + 1) are strictly increasing
        // across blocks, so the cursor only advances forward.
        let mut txout_cursor = indexer.vecs.transactions.first_txoutindex.cursor();

        for (idx, _h) in range.enumerate() {
            let first_txindex = first_txindexes[idx];
            let next_first_txindex = first_txindexes
                .get(idx + 1)
                .copied()
                .unwrap_or(TxIndex::from(total_txs));

            let out_start = if first_txindex.to_usize() + 1 < next_first_txindex.to_usize() {
                let target = first_txindex.to_usize() + 1;
                txout_cursor.advance(target - txout_cursor.position());
                txout_cursor.next().unwrap().to_usize()
            } else {
                out_firsts
                    .get(idx + 1)
                    .copied()
                    .unwrap_or(TxOutIndex::from(total_outputs))
                    .to_usize()
            };
            let out_end = out_firsts
                .get(idx + 1)
                .copied()
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize();

            let values: Vec<Sats> = indexer
                .vecs
                .outputs
                .value
                .collect_range_at(out_start, out_end);
            let output_types: Vec<OutputType> = indexer
                .vecs
                .outputs
                .outputtype
                .collect_range_at(out_start, out_end);

            let mut hist = [0u32; NUM_BINS];
            for i in 0..values.len() {
                if let Some(bin) = oracle.output_to_bin(values[i], output_types[i]) {
                    hist[bin] += 1;
                }
            }

            ref_bins.push(oracle.process_histogram(&hist));
        }

        ref_bins
    }
}

impl<M: StorageMode> Vecs<M> {
    /// Returns an Oracle seeded from the last committed price, with the last
    /// window_size blocks already processed. Ready for additional blocks (e.g. mempool).
    pub fn live_oracle<IM: StorageMode>(&self, indexer: &Indexer<IM>) -> Result<Oracle> {
        let config = Config::default();
        let height = indexer.vecs.blocks.timestamp.len();
        let last_cents = self
            .price
            .collect_one_at(self.price.len() - 1)
            .unwrap();
        let seed_bin = cents_to_bin(last_cents.inner() as f64);
        let window_size = config.window_size;
        let oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Vecs::feed_blocks(o, indexer, height.saturating_sub(window_size)..height);
        });

        Ok(oracle)
    }
}
