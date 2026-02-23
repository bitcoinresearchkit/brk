use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, Price};

/// Price range and choppiness metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_min: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_1w_max: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_2w_min: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_2w_max: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_1m_min: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_1m_max: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_1y_min: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_1y_max: Price<ComputedFromHeightLast<Dollars, M>>,
    pub price_true_range: ComputedFromHeightLast<StoredF32, M>,
    pub price_true_range_2w_sum: ComputedFromHeightLast<StoredF32, M>,
    pub price_2w_choppiness_index: ComputedFromHeightLast<StoredF32, M>,
}
