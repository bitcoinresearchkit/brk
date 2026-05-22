//! Mempool info + price-blending output histogram.

use brk_oracle::HistogramRaw;
use brk_types::MempoolInfo;

use crate::Mempool;

impl Mempool {
    #[must_use]
    pub fn info(&self) -> MempoolInfo {
        self.read().info.clone()
    }

    /// Snapshot of pre-bucketed oracle bins across all live mempool tx
    /// outputs. The total is maintained incrementally by `TxStore` on
    /// every insert/remove, so this hot path is `O(NUM_BINS)` regardless
    /// of pool size. Used by `live_price` to blend the mempool into the
    /// committed oracle without re-parsing scripts per request.
    #[must_use]
    pub fn live_histogram(&self) -> HistogramRaw {
        self.read().txs.live_histogram()
    }
}
