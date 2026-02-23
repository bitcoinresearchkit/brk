use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, ComputedFromHeightRatio, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub vaulted_price_ratio: ComputedFromHeightRatio<M>,
    pub active_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub active_price_ratio: ComputedFromHeightRatio<M>,
    pub true_market_mean: Price<ComputedFromHeightLast<Dollars, M>>,
    pub true_market_mean_ratio: ComputedFromHeightRatio<M>,
    pub cointime_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub cointime_price_ratio: ComputedFromHeightRatio<M>,
}
