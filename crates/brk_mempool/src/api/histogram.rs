//! Mempool info + price-blending output histogram.

use brk_error::Result;
use brk_oracle::HistogramRaw;
use brk_types::{BlockHash, MempoolInfo};

use crate::Mempool;

impl Mempool {
    #[must_use]
    pub fn info(&self) -> Result<MempoolInfo> {
        let state = self.read();
        state.ensure_published()?;
        Ok(state.info.clone())
    }

    /// Snapshot of pre-bucketed round-dollar-eligible bins across all live
    /// mempool tx outputs. Maintained incrementally by `TxStore` on every
    /// insert/remove, so this hot path is `O(NUM_BINS)` regardless of pool
    /// size. Used by `live_price` to blend the mempool into the committed
    /// oracle without re-parsing scripts per request. Requires a completed
    /// publication at the requested confirmed-chain tip.
    pub fn live_eligible_histogram(&self, tip: &BlockHash) -> Result<HistogramRaw> {
        let state = self.read();
        state.ensure_published_at(tip)?;
        Ok(state.txs.live_eligible_histogram())
    }

    /// Snapshot of the raw histogram: every live mempool output binned by
    /// value with no payment filtering. Backs the `histogram/raw/live`
    /// endpoint. Requires a completed publication at the requested tip.
    pub fn live_raw_histogram(&self, tip: &BlockHash) -> Result<HistogramRaw> {
        let state = self.read();
        state.ensure_published_at(tip)?;
        Ok(state.txs.live_raw_histogram())
    }
}

#[cfg(test)]
#[path = "../../tests/unit/api/histogram.rs"]
mod tests;
