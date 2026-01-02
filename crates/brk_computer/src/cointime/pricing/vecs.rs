use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::{ComputedRatioVecsFromDateIndex, ComputedVecsFromHeight};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_vaulted_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_vaulted_price_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_active_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_active_price_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_true_market_mean: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_true_market_mean_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_cointime_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_cointime_price_ratio: ComputedRatioVecsFromDateIndex,
}
