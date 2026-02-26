use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, ComputedFromHeightRatioExtended, Price};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub vaulted_price_ratio: ComputedFromHeightRatioExtended<M>,
    pub active_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub active_price_ratio: ComputedFromHeightRatioExtended<M>,
    pub true_market_mean: Price<ComputedFromHeightLast<Dollars, M>>,
    pub true_market_mean_ratio: ComputedFromHeightRatioExtended<M>,
    pub cointime_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub cointime_price_ratio: ComputedFromHeightRatioExtended<M>,
}
