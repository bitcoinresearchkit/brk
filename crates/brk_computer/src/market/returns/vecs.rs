use brk_traversable::Traversable;
use brk_types::{Close, DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::{
    internal::{BinaryDateLast, ComputedDateLast, ComputedStandardDeviationVecsDate},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

/// Price returns, CAGR, and returns standard deviation metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    // KISS: Price returns (lazy, from price.close and lookback.price_ago)
    pub price_returns: ByLookbackPeriod<BinaryDateLast<StoredF32, Close<Dollars>, Dollars>>,

    // CAGR (computed from returns, 2y+ only)
    pub cagr: ByDcaCagr<ComputedDateLast<StoredF32>>,

    // Returns standard deviation (computed from 1d returns)
    pub indexes_to_1d_returns_1w_sd: ComputedStandardDeviationVecsDate,
    pub indexes_to_1d_returns_1m_sd: ComputedStandardDeviationVecsDate,
    pub indexes_to_1d_returns_1y_sd: ComputedStandardDeviationVecsDate,

    // Downside returns and deviation (for Sortino ratio)
    pub dateindex_to_downside_returns: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_downside_1w_sd: ComputedStandardDeviationVecsDate,
    pub indexes_to_downside_1m_sd: ComputedStandardDeviationVecsDate,
    pub indexes_to_downside_1y_sd: ComputedStandardDeviationVecsDate,
}
