use brk_traversable::Traversable;
use brk_types::{Close, DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::{
    internal::{ComputedFromDateLast, ComputedFromDateStdDev, LazyBinaryFromDateLast},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

/// Price returns, CAGR, and returns standard deviation metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_returns: ByLookbackPeriod<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,

    // CAGR (computed from returns, 2y+ only)
    pub cagr: ByDcaCagr<ComputedFromDateLast<StoredF32>>,

    // Returns standard deviation (computed from 1d returns)
    pub _1d_returns_1w_sd: ComputedFromDateStdDev,
    pub _1d_returns_1m_sd: ComputedFromDateStdDev,
    pub _1d_returns_1y_sd: ComputedFromDateStdDev,

    // Downside returns and deviation (for Sortino ratio)
    pub downside_returns: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub downside_1w_sd: ComputedFromDateStdDev,
    pub downside_1m_sd: ComputedFromDateStdDev,
    pub downside_1y_sd: ComputedFromDateStdDev,
}
