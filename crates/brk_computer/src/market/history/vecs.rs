use brk_traversable::Traversable;
use brk_types::{Close, Dollars, StoredF32};

use crate::grouped::{ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex};

/// Historical price lookback, returns, and CAGR metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1d_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1w_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_3m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_6m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_2y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_3y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_4y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_5y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_6y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_8y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_10y_ago: ComputedVecsFromDateIndex<Dollars>,

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

    pub _2y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_cagr: ComputedVecsFromDateIndex<StoredF32>,
}
