use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedBlockLast, ComputedRatioVecsDate};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vaulted_price: ComputedBlockLast<Dollars>,
    pub vaulted_price_ratio: ComputedRatioVecsDate,
    pub active_price: ComputedBlockLast<Dollars>,
    pub active_price_ratio: ComputedRatioVecsDate,
    pub true_market_mean: ComputedBlockLast<Dollars>,
    pub true_market_mean_ratio: ComputedRatioVecsDate,
    pub cointime_price: ComputedBlockLast<Dollars>,
    pub cointime_price_ratio: ComputedRatioVecsDate,
}
