use brk_traversable::Traversable;
use brk_types::{Close, DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::{
    internal::{ComputedDateLast, ComputedStandardDeviationVecsDate, LazyBinaryDateLast},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

/// Price returns, CAGR, and returns standard deviation metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_returns: ByLookbackPeriod<LazyBinaryDateLast<StoredF32, Close<Dollars>, Dollars>>,

    // CAGR (computed from returns, 2y+ only)
    pub cagr: ByDcaCagr<ComputedDateLast<StoredF32>>,

    // Returns standard deviation (computed from 1d returns)
    pub _1d_returns_1w_sd: ComputedStandardDeviationVecsDate,
    pub _1d_returns_1m_sd: ComputedStandardDeviationVecsDate,
    pub _1d_returns_1y_sd: ComputedStandardDeviationVecsDate,

    // Downside returns and deviation (for Sortino ratio)
    pub downside_returns: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub downside_1w_sd: ComputedStandardDeviationVecsDate,
    pub downside_1m_sd: ComputedStandardDeviationVecsDate,
    pub downside_1y_sd: ComputedStandardDeviationVecsDate,
}
