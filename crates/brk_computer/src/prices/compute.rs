use std::ops::Range;

use brk_error::Result;
use brk_indexer::{Indexer, Lengths};
use brk_oracle::{
    Config, HistogramRaw, Oracle, START_HEIGHT_FAST, START_HEIGHT_SLOW, bin_to_cents, cents_to_bin,
    for_each_round_dollar_bin,
};
use brk_types::{Cents, OutputType, Sats, TxIndex, TxOutIndex};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, StorageMode, VecIndex, WritableVec};

use super::Vecs;
use crate::indexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let starting_lengths = indexer.safe_lengths();

        self.compute_prices(indexer, exit)?;
        self.split.open.cents.compute_first(
            &starting_lengths,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.split.high.cents.compute_max(
            &starting_lengths,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.split.low.cents.compute_min(
            &starting_lengths,
            &self.spot.cents.height,
            indexes,
            exit,
        )?;
        self.ohlc.cents.compute_from_split(
            &starting_lengths,
            indexes,
            &self.split.open.cents,
            &self.split.high.cents,
            &self.split.low.cents,
            &self.split.close.cents,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }

    fn compute_prices(&mut self, indexer: &Indexer, exit: &Exit) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;

        let source_version =
            indexer.vecs.outputs.value.version() + indexer.vecs.outputs.output_type.version();
        self.spot
            .cents
            .height
            .inner
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        if total_heights <= START_HEIGHT_SLOW {
            return Ok(());
        }

        // Reorg: truncate to starting_lengths
        let truncate_to = self.spot.cents.height.len().min(starting_height.to_usize());
        self.spot
            .cents
            .height
            .inner
            .truncate_if_needed_at(truncate_to)?;

        if self.spot.cents.height.len() < START_HEIGHT_SLOW {
            for line in brk_oracle::PRICES
                .lines()
                .skip(self.spot.cents.height.len())
            {
                if self.spot.cents.height.len() >= START_HEIGHT_SLOW {
                    break;
                }
                let dollars: f64 = line.parse().unwrap_or(0.0);
                let cents = (dollars * 100.0).round() as u64;
                self.spot.cents.height.inner.push(Cents::new(cents));
            }
        }

        if self.spot.cents.height.len() >= total_heights {
            return Ok(());
        }

        let committed = self.spot.cents.height.len();
        let config = Config::for_height(committed);
        let prev_cents = self
            .spot
            .cents
            .height
            .collect_one_at(committed - 1)
            .unwrap();
        let seed_bin = cents_to_bin(prev_cents.inner() as f64);
        let warmup = config.window_size.min(committed - START_HEIGHT_SLOW);
        let mut oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Self::feed_blocks(o, indexer, (committed - warmup)..committed, None);
        });

        let num_new = total_heights - committed;
        info!(
            "Computing oracle prices: {} to {} ({warmup} warmup)",
            committed, total_heights
        );

        // Slow cold-start EMA up to START_HEIGHT_FAST, then switch to the fast
        // mature-market EMA. Steady-state runs start past START_HEIGHT_FAST and skip
        // the slow segment entirely.
        let mut ref_bins = Vec::with_capacity(num_new);
        if committed < START_HEIGHT_FAST {
            let slow_end = START_HEIGHT_FAST.min(total_heights);
            ref_bins.extend(Self::feed_blocks(&mut oracle, indexer, committed..slow_end, None));
            if slow_end == START_HEIGHT_FAST {
                oracle.reconfigure(Config::default());
            }
        }
        let fast_start = committed.max(START_HEIGHT_FAST);
        if fast_start < total_heights {
            ref_bins.extend(Self::feed_blocks(
                &mut oracle,
                indexer,
                fast_start..total_heights,
                None,
            ));
        }

        for (i, ref_bin) in ref_bins.into_iter().enumerate() {
            self.spot
                .cents
                .height
                .inner
                .push(Cents::new(bin_to_cents(ref_bin)));

            let progress = ((i + 1) * 100 / num_new) as u8;
            if i > 0 && progress > ((i * 100 / num_new) as u8) {
                info!("Oracle price computation: {}%", progress);
            }
        }

        {
            let _lock = exit.lock();
            self.spot.cents.height.inner.write()?;
        }

        info!(
            "Oracle prices complete: {} committed",
            self.spot.cents.height.len()
        );

        Ok(())
    }

    /// Feed a range of blocks from the indexer into an Oracle (skipping coinbase),
    /// returning per-block ref_bin values. Outputs are grouped per transaction
    /// because `for_each_round_dollar_bin` drops a whole tx on any OP_RETURN.
    ///
    /// Pass `cap = None` from compute paths, when the indexer is quiescent and
    /// raw vec lengths are authoritative. Pass `cap = Some(&safe_lengths)` from
    /// reader paths so concurrent writer pushes past the cap are invisible.
    pub fn feed_blocks<IM: StorageMode>(
        oracle: &mut Oracle,
        indexer: &Indexer<IM>,
        range: Range<usize>,
        cap: Option<&Lengths>,
    ) -> Vec<f64> {
        let mut ref_bins = Vec::with_capacity(range.len());
        Self::feed_blocks_with(oracle, indexer, range, cap, |_, _, ref_bin| {
            ref_bins.push(ref_bin);
        });
        ref_bins
    }

    /// Feed a range of blocks into an Oracle and call `on_block` after each
    /// processed block. This lets callers observe derived state such as EMA
    /// without duplicating the histogram extraction path.
    pub fn feed_blocks_with<IM: StorageMode>(
        oracle: &mut Oracle,
        indexer: &Indexer<IM>,
        range: Range<usize>,
        cap: Option<&Lengths>,
        mut on_block: impl FnMut(usize, &Oracle, f64),
    ) {
        let (total_txs, total_outputs, height_len) = match cap {
            Some(c) => (
                c.tx_index.to_usize(),
                c.txout_index.to_usize(),
                c.height.to_usize(),
            ),
            None => (
                indexer.vecs.transactions.txid.len(),
                indexer.vecs.outputs.value.len(),
                indexer.vecs.transactions.first_tx_index.len(),
            ),
        };

        // Pre-collect height-indexed data for the range (plus one extra for next-block lookups)
        let collect_end = (range.end + 1).min(height_len);
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

        // Cursor avoids per-block PcoVec page decompression for the
        // tx-indexed first_txout_index lookup. Accessed tx_index values
        // are strictly increasing across blocks, so it only advances forward.
        let mut txout_cursor = indexer.vecs.transactions.first_txout_index.cursor();

        // Reusable buffers: avoid per-block allocation. `tx_starts` holds the
        // first txout index of each non-coinbase tx in the current block.
        let mut values: Vec<Sats> = Vec::new();
        let mut output_types: Vec<OutputType> = Vec::new();
        let mut tx_starts: Vec<usize> = Vec::new();

        for idx in 0..range.len() {
            let next_first_tx_index = first_tx_indexes
                .get(idx + 1)
                .copied()
                .unwrap_or(TxIndex::from(total_txs))
                .to_usize();
            let block_first_tx = first_tx_indexes[idx].to_usize() + 1;
            let tx_count = next_first_tx_index - block_first_tx;

            let out_end = out_firsts
                .get(idx + 1)
                .copied()
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize();

            txout_cursor.advance(block_first_tx - txout_cursor.position());
            tx_starts.clear();
            for _ in 0..tx_count {
                tx_starts.push(txout_cursor.next().unwrap().to_usize());
            }
            let out_start = tx_starts.first().copied().unwrap_or(out_end);

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

            let mut hist = HistogramRaw::zeros();
            for tx in 0..tx_count {
                let lo = tx_starts[tx] - out_start;
                let hi = tx_starts
                    .get(tx + 1)
                    .map(|s| s - out_start)
                    .unwrap_or(out_end - out_start);
                let outputs = values[lo..hi]
                    .iter()
                    .copied()
                    .zip(output_types[lo..hi].iter().copied());
                for_each_round_dollar_bin(range.start + idx, outputs, |bin| {
                    hist.increment(bin as usize)
                });
            }

            let ref_bin = oracle.process_histogram(&hist);
            on_block(range.start + idx, oracle, ref_bin);
        }
    }
}
