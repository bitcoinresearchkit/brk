use std::ops::Range;

use brk_error::Result;
use brk_indexer::{Indexer, Lengths};
use brk_oracle::{Config, Histogram, Oracle, START_HEIGHT, bin_to_cents, cents_to_bin};
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

        if total_heights <= START_HEIGHT {
            return Ok(());
        }

        // Reorg: truncate to starting_lengths
        let truncate_to = self.spot.cents.height.len().min(starting_height.to_usize());
        self.spot
            .cents
            .height
            .inner
            .truncate_if_needed_at(truncate_to)?;

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
                self.spot.cents.height.inner.push(Cents::new(cents));
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
            Self::feed_blocks(o, indexer, (committed - warmup)..committed, None);
        });

        let num_new = total_heights - committed;
        info!(
            "Computing oracle prices: {} to {} ({warmup} warmup)",
            committed, total_heights
        );

        let ref_bins =
            Self::feed_blocks(&mut oracle, indexer, committed..total_heights, None);

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
    /// returning per-block ref_bin values.
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

        let mut ref_bins = Vec::with_capacity(range.len());

        // Cursor avoids per-block PcoVec page decompression for the
        // tx-indexed first_txout_index lookup. Accessed tx_index values
        // (first_tx_index + 1) are strictly increasing across blocks,
        // so the cursor only advances forward.
        let mut txout_cursor = indexer.vecs.transactions.first_txout_index.cursor();

        // Reusable buffers: avoid per-block allocation
        let mut values: Vec<Sats> = Vec::new();
        let mut output_types: Vec<OutputType> = Vec::new();

        for idx in 0..range.len() {
            let first_tx_index = first_tx_indexes[idx];
            let next_first_tx_index = first_tx_indexes
                .get(idx + 1)
                .copied()
                .unwrap_or(TxIndex::from(total_txs));

            let out_end = out_firsts
                .get(idx + 1)
                .copied()
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize();
            let out_start = if first_tx_index.to_usize() + 1 < next_first_tx_index.to_usize() {
                let target = first_tx_index.to_usize() + 1;
                txout_cursor.advance(target - txout_cursor.position());
                txout_cursor.next().unwrap().to_usize()
            } else {
                out_end
            };

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

            let mut hist = Histogram::zeros();
            for (sats, output_type) in values.iter().zip(&output_types) {
                if let Some(bin) = oracle.output_to_bin(*sats, *output_type) {
                    hist.increment(bin);
                }
            }

            ref_bins.push(oracle.process_histogram(&hist));
        }

        ref_bins
    }
}
