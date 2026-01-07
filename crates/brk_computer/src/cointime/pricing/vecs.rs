use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedRatioVecsDate, ComputedBlockLast};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_vaulted_price: ComputedBlockLast<Dollars>,
    pub indexes_to_vaulted_price_ratio: ComputedRatioVecsDate,
    pub indexes_to_active_price: ComputedBlockLast<Dollars>,
    pub indexes_to_active_price_ratio: ComputedRatioVecsDate,
    pub indexes_to_true_market_mean: ComputedBlockLast<Dollars>,
    pub indexes_to_true_market_mean_ratio: ComputedRatioVecsDate,
    pub indexes_to_cointime_price: ComputedBlockLast<Dollars>,
    pub indexes_to_cointime_price_ratio: ComputedRatioVecsDate,
}
