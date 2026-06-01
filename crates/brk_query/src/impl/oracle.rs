use std::{ops::Range, sync::Arc};

use brk_computer::prices::Vecs as PricesVecs;
use brk_error::{Error, Result};
use brk_indexer::Lengths;
use brk_oracle::{
    Config, HistogramEma, HistogramEmaCompact, HistogramRaw, Oracle, START_HEIGHT_FAST,
    cents_to_bin, sats_to_bin,
};
use brk_types::{Day1, Dollars, Sats, TxOutIndex};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn live_price(&self) -> Result<Dollars> {
        Ok(self.live_oracle()?.price_dollars())
    }

    /// Smoothed EMA histogram at the live tip, quantized for the wire.
    pub fn live_histogram_ema(&self) -> Result<HistogramEmaCompact> {
        Ok(self.live_oracle()?.ema().to_compact())
    }

    /// Smoothed EMA histogram for a confirmed `height`, deterministically
    /// reconstructed by replaying the window ending at `height`. EMA values are
    /// seed-independent, so the result is exact.
    pub fn confirmed_histogram_ema(&self, height: usize) -> Result<HistogramEmaCompact> {
        let safe = self.check_histogram_height(height)?;
        Ok(self.ema_oracle_at(height, &safe)?.ema().to_compact())
    }

    /// Smoothed EMA histogram for a calendar `day`: the bin-by-bin average of
    /// every confirmed block's per-block EMA. Each block's EMA is reconstructed
    /// independently (seed-independent, so exact); averaging keeps the result an
    /// intensive per-block rate rather than letting a busy day dominate.
    pub fn confirmed_histogram_ema_day(&self, day: Day1) -> Result<HistogramEmaCompact> {
        let safe = self.safe_lengths();
        let range = self.day_block_range(day, &safe)?;
        Ok(self.average_histogram_ema_range(range, &safe)?.to_compact())
    }

    fn average_histogram_ema_range(
        &self,
        range: Range<usize>,
        safe: &Lengths,
    ) -> Result<HistogramEma> {
        let count = range.len();
        let mut acc = HistogramEma::zeros();

        for segment in ema_config_segments(range) {
            let mut oracle = self.ema_oracle_at(segment.start, safe)?;
            add_ema(&mut acc, oracle.ema());

            let feed_start = segment.start + 1;
            if feed_start < segment.end {
                PricesVecs::feed_blocks_with(
                    &mut oracle,
                    self.indexer(),
                    feed_start..segment.end,
                    Some(safe),
                    |_, oracle, _| add_ema(&mut acc, oracle.ema()),
                );
            }
        }

        let count = count as f64;
        acc.iter_mut().for_each(|a| *a /= count);
        Ok(acc)
    }

    /// Unfiltered per-bin output counts at the live tip: every forming-block
    /// mempool output binned by value, with none of the round-dollar payment
    /// filters applied. Zeros when no mempool is configured.
    pub fn live_histogram_raw(&self) -> Result<HistogramRaw> {
        Ok(match self.mempool() {
            Some(mempool) => mempool.live_raw_histogram(),
            None => HistogramRaw::zeros(),
        })
    }

    /// Unfiltered per-bin output counts for a confirmed `height`: every output
    /// in the block binned by value, with no payment filtering.
    pub fn confirmed_histogram_raw(&self, height: usize) -> Result<HistogramRaw> {
        let safe = self.check_histogram_height(height)?;
        Ok(self.block_raw_histogram(height, &safe))
    }

    /// Unfiltered per-bin output counts for a calendar `day`: every block's raw
    /// histogram summed bin-by-bin. Raw counts are additive, so the day total is
    /// just the sum across its confirmed blocks.
    pub fn confirmed_histogram_raw_day(&self, day: Day1) -> Result<HistogramRaw> {
        let safe = self.safe_lengths();
        let range = self.day_block_range(day, &safe)?;
        let mut acc = HistogramRaw::zeros();
        for height in range {
            let block = self.block_raw_histogram(height, &safe);
            acc.iter_mut()
                .zip(block.iter())
                .for_each(|(a, &v)| *a += v);
        }
        Ok(acc)
    }

    /// The live tip oracle: the cached committed base, with the forming block's
    /// mempool outputs blended in as a final slot when a mempool is configured.
    fn live_oracle(&self) -> Result<Oracle> {
        let mut oracle = (*self.cached_oracle()?).clone();
        if let Some(mempool) = self.mempool() {
            oracle.process_histogram(&mempool.live_eligible_histogram());
        }
        Ok(oracle)
    }

    /// Tip oracle warmed over the last `window_size` committed blocks, seeded
    /// from the last committed price. Cached per tip height; rebuilt on advance
    /// or reorg.
    fn cached_oracle(&self) -> Result<Arc<Oracle>> {
        let safe = self.safe_lengths();
        let height = safe.height;

        if let Some(oracle) = self
            .0
            .live_oracle
            .read()
            .unwrap()
            .as_ref()
            .filter(|(h, _)| *h == height)
            .map(|(_, o)| o.clone())
        {
            return Ok(oracle);
        }

        let last = self.computer().prices.spot.cents.height.len().saturating_sub(1);
        let seed_bin = self.seed_bin_at(last)?;
        let oracle = Arc::new(self.warm_oracle(seed_bin, height.to_usize(), &safe));

        let mut cache = self.0.live_oracle.write().unwrap();
        if cache.as_ref().is_none_or(|(h, _)| *h != height) {
            *cache = Some((height, oracle.clone()));
        }
        Ok(oracle)
    }

    /// Oracle warmed to just after `height`, ready for its per-block EMA. Seeds
    /// from the stored spot price at `height`, though the EMA is seed-independent
    /// so the seed only sets the price read-out, not the window contents.
    fn ema_oracle_at(&self, height: usize, safe: &Lengths) -> Result<Oracle> {
        let seed_bin = self.seed_bin_at(height)?;
        Ok(self.warm_oracle(seed_bin, height + 1, safe))
    }

    /// An oracle seeded at `seed_bin` and warmed by replaying the `window_size`
    /// committed blocks ending just before `end`. Reads are capped at `safe` so
    /// concurrent indexer writes past the cap stay invisible.
    fn warm_oracle(&self, seed_bin: f64, end: usize, safe: &Lengths) -> Oracle {
        let config = Config::for_height(end.saturating_sub(1));
        let start = end.saturating_sub(config.window_size);
        Oracle::from_checkpoint(seed_bin, config, |o| {
            PricesVecs::feed_blocks_with(o, self.indexer(), start..end, Some(safe), |_, _, _| {});
        })
    }

    /// Seed bin for an oracle warm-up: the stored spot price at `height` mapped
    /// `cents -> bin`. 404s when the oracle prices aren't computed that far yet,
    /// which also covers the stamp-before-write race where the vec length leads
    /// the readable data.
    fn seed_bin_at(&self, height: usize) -> Result<f64> {
        let cents = self
            .computer()
            .prices
            .spot
            .cents
            .height
            .collect_one_at(height)
            .ok_or_else(|| Error::NotFound("oracle prices not yet computed".to_string()))?;
        Ok(cents_to_bin(cents.inner() as f64))
    }

    /// `height < min(spot price len, safe height)` or 404.
    /// Returns the safe lengths so callers cap reads at the same bound.
    fn check_histogram_height(&self, height: usize) -> Result<Lengths> {
        let safe = self.safe_lengths();
        let bound = self
            .computer()
            .prices
            .spot
            .cents
            .height
            .len()
            .min(safe.height.to_usize());
        if height >= bound {
            return Err(Error::NotFound(format!(
                "oracle histogram unavailable for height {height}"
            )));
        }
        Ok(safe)
    }

    /// The confirmed block heights `[first, end)` of calendar `day`, clamped to
    /// the same histogram-available bound as `check_histogram_height`. 404 when
    /// the day has no committed blocks in range.
    fn day_block_range(&self, day: Day1, safe: &Lengths) -> Result<Range<usize>> {
        let first_height = &self.computer().indexes.day1.first_height;
        let bound = self
            .computer()
            .prices
            .spot
            .cents
            .height
            .len()
            .min(safe.height.to_usize());
        let start = first_height
            .collect_one(day)
            .map_or(usize::MAX, |h| h.to_usize());
        let end = first_height
            .collect_one(day + 1)
            .map_or(bound, |h| h.to_usize())
            .min(bound);
        if start >= end {
            return Err(Error::NotFound(format!(
                "oracle histogram unavailable for day {day}"
            )));
        }
        Ok(start..end)
    }

    /// One confirmed block's unfiltered histogram: every output in the block,
    /// coinbase included, binned by value via `sats_to_bin` with no payment
    /// filtering. Built from a single batched columnar read of the block's
    /// output-value range.
    fn block_raw_histogram(&self, height: usize, safe: &Lengths) -> HistogramRaw {
        let indexer = self.indexer();
        let total_outputs = safe.txout_index.to_usize();
        let next_height = (height + 2).min(safe.height.to_usize());

        let out_firsts: Vec<TxOutIndex> = indexer
            .vecs
            .outputs
            .first_txout_index
            .collect_range_at(height, next_height);
        let out_start = out_firsts[0].to_usize();
        let out_end = out_firsts
            .get(1)
            .copied()
            .unwrap_or(TxOutIndex::from(total_outputs))
            .to_usize();

        let mut hist = HistogramRaw::zeros();
        let values: Vec<Sats> = indexer.vecs.outputs.value.collect_range_at(out_start, out_end);
        for sats in values {
            if let Some(bin) = sats_to_bin(sats) {
                hist.increment(bin);
            }
        }
        hist
    }
}

fn add_ema(acc: &mut HistogramEma, ema: &HistogramEma) {
    acc.iter_mut()
        .zip(ema.iter())
        .for_each(|(a, &e)| *a += e);
}

fn ema_config_segments(range: Range<usize>) -> impl Iterator<Item = Range<usize>> {
    let split = START_HEIGHT_FAST.clamp(range.start, range.end);
    [range.start..split, split..range.end]
        .into_iter()
        .filter(|range| !range.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ema_config_segments_split_at_fast_start() {
        let segments: Vec<_> =
            ema_config_segments((START_HEIGHT_FAST - 2)..(START_HEIGHT_FAST + 2)).collect();
        assert_eq!(
            segments,
            vec![
                (START_HEIGHT_FAST - 2)..START_HEIGHT_FAST,
                START_HEIGHT_FAST..(START_HEIGHT_FAST + 2),
            ]
        );
    }

    #[test]
    fn ema_config_segments_omits_empty_sides() {
        let slow: Vec<_> = ema_config_segments((START_HEIGHT_FAST - 2)..START_HEIGHT_FAST).collect();
        assert_eq!(slow, vec![(START_HEIGHT_FAST - 2)..START_HEIGHT_FAST]);

        let fast: Vec<_> =
            ema_config_segments(START_HEIGHT_FAST..(START_HEIGHT_FAST + 2)).collect();
        assert_eq!(fast, vec![START_HEIGHT_FAST..(START_HEIGHT_FAST + 2)]);
    }
}
