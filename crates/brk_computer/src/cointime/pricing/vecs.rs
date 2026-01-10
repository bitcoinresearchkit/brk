use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedFromHeightLast, ComputedFromDateRatio};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vaulted_price: ComputedFromHeightLast<Dollars>,
    pub vaulted_price_ratio: ComputedFromDateRatio,
    pub active_price: ComputedFromHeightLast<Dollars>,
    pub active_price_ratio: ComputedFromDateRatio,
    pub true_market_mean: ComputedFromHeightLast<Dollars>,
    pub true_market_mean_ratio: ComputedFromDateRatio,
    pub cointime_price: ComputedFromHeightLast<Dollars>,
    pub cointime_price_ratio: ComputedFromDateRatio,
}
