use std::sync::Arc;

use brk_computer::prices::Vecs as PricesVecs;
use brk_error::{Error, Result};
use brk_indexer::Lengths;
use brk_oracle::{
    Config, HistogramEmaCompact, HistogramRaw, Oracle, START_HEIGHT, cents_to_bin,
    for_each_round_dollar_bin,
};
use brk_types::{Dollars, OutputType, Sats, TxIndex, TxOutIndex};
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
        let ref_bin = self.seed_bin_at(height)?;
        Ok(self.warm_oracle(ref_bin, height + 1, &safe).ema().to_compact())
    }

    /// Un-smoothed per-block round-dollar counts at the live tip: the mempool's
    /// forming-block histogram, or zeros when no mempool is configured.
    pub fn live_histogram_raw(&self) -> Result<HistogramRaw> {
        Ok(match self.mempool() {
            Some(mempool) => mempool.live_histogram(),
            None => HistogramRaw::zeros(),
        })
    }

    /// Un-smoothed per-block round-dollar counts for a confirmed `height`.
    pub fn confirmed_histogram_raw(&self, height: usize) -> Result<HistogramRaw> {
        let safe = self.check_histogram_height(height)?;
        Ok(self.block_raw_histogram(height, &safe))
    }

    /// The live tip oracle: the cached committed base, with the forming block's
    /// mempool outputs blended in as a final slot when a mempool is configured.
    fn live_oracle(&self) -> Result<Oracle> {
        let mut oracle = (*self.cached_oracle()?).clone();
        if let Some(mempool) = self.mempool() {
            oracle.process_histogram(&mempool.live_histogram());
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

    /// An oracle seeded at `seed_bin` and warmed by replaying the `window_size`
    /// committed blocks ending just before `end`. Reads are capped at `safe` so
    /// concurrent indexer writes past the cap stay invisible.
    fn warm_oracle(&self, seed_bin: f64, end: usize, safe: &Lengths) -> Oracle {
        let config = Config::default();
        let start = end.saturating_sub(config.window_size);
        Oracle::from_checkpoint(seed_bin, config, |o| {
            PricesVecs::feed_blocks(o, self.indexer(), start..end, Some(safe));
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

    /// `START_HEIGHT <= height < min(spot price len, safe height)` or 404.
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
        if height < START_HEIGHT || height >= bound {
            return Err(Error::NotFound(format!(
                "oracle histogram unavailable for height {height}"
            )));
        }
        Ok(safe)
    }

    /// One confirmed block's round-dollar histogram, built from batched columnar
    /// reads and the shared `for_each_round_dollar_bin` filter. Kept separate from
    /// the hot-path `feed_blocks` (cursor + reusable buffers over a block range).
    fn block_raw_histogram(&self, height: usize, safe: &Lengths) -> HistogramRaw {
        let indexer = self.indexer();
        let total_txs = safe.tx_index.to_usize();
        let total_outputs = safe.txout_index.to_usize();
        let next_height = (height + 2).min(safe.height.to_usize());

        let first_tx_indexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_range_at(height, next_height);
        let out_firsts: Vec<TxOutIndex> = indexer
            .vecs
            .outputs
            .first_txout_index
            .collect_range_at(height, next_height);

        let block_first_tx = first_tx_indexes[0].to_usize() + 1;
        let next_first_tx = first_tx_indexes
            .get(1)
            .copied()
            .unwrap_or(TxIndex::from(total_txs))
            .to_usize();
        let tx_count = next_first_tx - block_first_tx;

        let mut hist = HistogramRaw::zeros();
        if tx_count == 0 {
            return hist;
        }

        let out_end = out_firsts
            .get(1)
            .copied()
            .unwrap_or(TxOutIndex::from(total_outputs))
            .to_usize();
        let tx_starts: Vec<usize> = indexer
            .vecs
            .transactions
            .first_txout_index
            .collect_range_at(block_first_tx, next_first_tx)
            .into_iter()
            .map(|t| t.to_usize())
            .collect();
        let out_start = tx_starts.first().copied().unwrap_or(out_end);

        let values: Vec<Sats> = indexer.vecs.outputs.value.collect_range_at(out_start, out_end);
        let output_types: Vec<OutputType> = indexer
            .vecs
            .outputs
            .output_type
            .collect_range_at(out_start, out_end);

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
            for_each_round_dollar_bin(height, outputs, |bin| hist.increment(bin as usize));
        }
        hist
    }
}
