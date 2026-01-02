use brk_traversable::Traversable;
use brk_types::{Close, DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{
    ComputedStandardDeviationVecsFromDateIndex, ComputedVecsFromDateIndex,
    LazyVecsFrom2FromDateIndex,
};

/// Price returns, CAGR, and returns standard deviation metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    // Price returns (lazy, from price.close and history.price_*_ago)
    pub _1d_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _1w_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _1m_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _3m_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _6m_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _1y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _2y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _3y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _4y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _5y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _6y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _8y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _10y_price_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,

    // CAGR (computed from returns)
    pub _2y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_cagr: ComputedVecsFromDateIndex<StoredF32>,

    // Returns standard deviation (computed from 1d returns)
    pub indexes_to_1d_returns_1w_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1m_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1y_sd: ComputedStandardDeviationVecsFromDateIndex,

    // Downside returns and deviation (for Sortino ratio)
    pub dateindex_to_downside_returns: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_downside_1w_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_downside_1m_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_downside_1y_sd: ComputedStandardDeviationVecsFromDateIndex,
}
