use std::ops::Range;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, START_HEIGHT, bin_to_cents, cents_to_bin};
use brk_types::{Cents, Indexes, OutputType, Sats, TxIndex, TxOutIndex};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, StorageMode, VecIndex, WritableVec};

use super::Vecs;
use crate::indexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_prices(indexer, starting_indexes, exit)?;
        self.split.open.cents.compute_first(
            starting_indexes,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.split.high.cents.compute_max(
            starting_indexes,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.split.low.cents.compute_min(
            starting_indexes,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.ohlc.cents.compute_from_split(
            starting_indexes,
            indexes,
            &self.split.open.cents,
            &self.split.high.cents,
            &self.split.low.cents,
            &self.split.close.cents,
            exit,
        )?;

        let _lock = exit.lock();
        self.db().compact()?;
        Ok(())
    }

    fn compute_prices(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version =
            indexer.vecs.outputs.value.version() + indexer.vecs.outputs.output_type.version();
        self.spot
            .cents
            .height
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        if total_heights <= START_HEIGHT {
            return Ok(());
        }

        // Reorg: truncate to starting_indexes
        let truncate_to = self
            .spot
            .cents
            .height
            .len()
            .min(starting_indexes.height.to_usize());
        self.spot.cents.height.truncate_if_needed_at(truncate_to)?;

        if self.spot.cents.height.len() < START_HEIGHT {
            for line in brk_oracle::PRICES
                .lines()
                .skip(self.spot.cents.height.len())
            {
                if self.spot.cents.height.len() >= START_HEIGHT {
                    break;
                }
                let dollars: f64 = line.parse().unwrap_or(0.0);
                let cents = (dollars * 100.0).round() as u64;
                self.spot.cents.height.push(Cents::new(cents));
            }
        }

        if self.spot.cents.height.len() >= total_heights {
            return Ok(());
        }

        let config = Config::default();
        let committed = self.spot.cents.height.len();
        let prev_cents = self
            .spot
            .cents
            .height
            .collect_one_at(committed - 1)
            .unwrap();
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
            self.spot
                .cents
                .height
                .push(Cents::new(bin_to_cents(ref_bin)));

            let progress = ((i + 1) * 100 / num_new) as u8;
            if i > 0 && progress > ((i * 100 / num_new) as u8) {
                info!("Oracle price computation: {}%", progress);
            }
        }

        {
            let _lock = exit.lock();
            self.spot.cents.height.write()?;
        }

        info!(
            "Oracle prices complete: {} committed",
            self.spot.cents.height.len()
        );

        Ok(())
    }

    /// Feed a range of blocks from the indexer into an Oracle (skipping coinbase),
    /// returning per-block ref_bin values.
    fn feed_blocks<M: StorageMode>(
        oracle: &mut Oracle,
        indexer: &Indexer<M>,
        range: Range<usize>,
    ) -> Vec<f64> {
        let total_txs = indexer.vecs.transactions.height.len();
        let total_outputs = indexer.vecs.outputs.value.len();

        // Pre-collect height-indexed data for the range (plus one extra for next-block lookups)
        let collect_end = (range.end + 1).min(indexer.vecs.transactions.first_tx_index.len());
        let first_tx_indexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_range_at(range.start, collect_end);

        let out_firsts: Vec<TxOutIndex> = indexer
            .vecs
            .outputs
            .first_txout_index
            .collect_range_at(range.start, collect_end);

        let mut ref_bins = Vec::with_capacity(range.len());

        // Cursor avoids per-block PcoVec page decompression for
        // the tx-indexed first_txout_index lookup.  The accessed
        // tx_index values (first_tx_index + 1) are strictly increasing
        // across blocks, so the cursor only advances forward.
        let mut txout_cursor = indexer.vecs.transactions.first_txout_index.cursor();

        // Reusable buffers — avoid per-block allocation
        let mut values: Vec<Sats> = Vec::new();
        let mut output_types: Vec<OutputType> = Vec::new();

        for (idx, _h) in range.enumerate() {
            let first_tx_index = first_tx_indexes[idx];
            let next_first_tx_index = first_tx_indexes
                .get(idx + 1)
                .copied()
                .unwrap_or(TxIndex::from(total_txs));

            let next_out_first = out_firsts
                .get(idx + 1)
                .copied()
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize();
            let out_start = if first_tx_index.to_usize() + 1 < next_first_tx_index.to_usize() {
                let target = first_tx_index.to_usize() + 1;
                txout_cursor.advance(target - txout_cursor.position());
                txout_cursor.next().unwrap().to_usize()
            } else {
                next_out_first
            };
            let out_end = next_out_first;

            indexer
                .vecs
                .outputs
                .value
                .collect_range_into_at(out_start, out_end, &mut values);
            indexer.vecs.outputs.output_type.collect_range_into_at(
                out_start,
                out_end,
                &mut output_types,
            );

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
            .spot
            .cents
            .height
            .collect_one_at(self.spot.cents.height.len() - 1)
            .unwrap();
        let seed_bin = cents_to_bin(last_cents.inner() as f64);
        let window_size = config.window_size;
        let oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Vecs::feed_blocks(o, indexer, height.saturating_sub(window_size)..height);
        });

        Ok(oracle)
    }
}
