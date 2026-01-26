use brk_traversable::Traversable;

use crate::internal::{ComputedFromDateRatio, PriceFromHeight};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vaulted_price: PriceFromHeight,
    pub vaulted_price_ratio: ComputedFromDateRatio,
    pub active_price: PriceFromHeight,
    pub active_price_ratio: ComputedFromDateRatio,
    pub true_market_mean: PriceFromHeight,
    pub true_market_mean_ratio: ComputedFromDateRatio,
    pub cointime_price: PriceFromHeight,
    pub cointime_price_ratio: ComputedFromDateRatio,
}
