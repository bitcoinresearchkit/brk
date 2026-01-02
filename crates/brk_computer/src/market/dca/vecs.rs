use brk_traversable::Traversable;
use brk_types::{Close, Dollars, Sats, StoredF32};

use crate::internal::{ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex};

/// Dollar-cost averaging metrics by time period and year class
#[derive(Clone, Traversable)]
pub struct Vecs {
    // DCA by period - stack
    pub _1w_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _1m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _3m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _6m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _1y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _2y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _3y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _4y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _5y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _6y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _8y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _10y_dca_stack: ComputedVecsFromDateIndex<Sats>,

    // DCA by period - avg price
    pub _1w_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _1m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _3m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _6m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _1y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _2y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _3y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _4y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _5y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _6y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _8y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _10y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,

    // DCA by period - returns (lazy)
    pub _1w_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _1m_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _3m_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _6m_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _1y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _2y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _3y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _4y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _5y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _6y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _8y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub _10y_dca_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,

    // DCA by period - CAGR
    pub _2y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,

    // DCA by year class - stack
    pub dca_class_2025_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2024_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2023_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2022_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2021_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2020_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2019_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2018_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2017_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2016_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2015_stack: ComputedVecsFromDateIndex<Sats>,

    // DCA by year class - avg price
    pub dca_class_2025_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2024_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2023_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2022_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2021_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2020_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2019_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2018_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2017_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2016_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2015_avg_price: ComputedVecsFromDateIndex<Dollars>,

    // DCA by year class - returns (lazy)
    pub dca_class_2025_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2024_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2023_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2022_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2021_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2020_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2019_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2018_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2017_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2016_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
    pub dca_class_2015_returns: LazyVecsFrom2FromDateIndex<StoredF32, Close<Dollars>, Dollars>,
}
