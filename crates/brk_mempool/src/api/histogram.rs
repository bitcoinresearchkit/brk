//! Mempool info + price-blending output histogram.

use brk_oracle::HistogramRaw;
use brk_types::MempoolInfo;

use crate::Mempool;

impl Mempool {
    #[must_use]
    pub fn info(&self) -> MempoolInfo {
        self.read().info.clone()
    }

    /// Snapshot of pre-bucketed round-dollar-eligible bins across all live
    /// mempool tx outputs. Maintained incrementally by `TxStore` on every
    /// insert/remove, so this hot path is `O(NUM_BINS)` regardless of pool
    /// size. Used by `live_price` to blend the mempool into the committed
    /// oracle without re-parsing scripts per request.
    #[must_use]
    pub fn live_eligible_histogram(&self) -> HistogramRaw {
        self.read().txs.live_eligible_histogram()
    }

    /// Snapshot of the raw histogram: every live mempool output binned by
    /// value with no payment filtering. Backs the `histogram/raw/live`
    /// endpoint.
    #[must_use]
    pub fn live_raw_histogram(&self) -> HistogramRaw {
        self.read().txs.live_raw_histogram()
    }
}
