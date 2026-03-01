use brk_traversable::Traversable;
use brk_types::{Cents, StoredF32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, Price};

/// Price range and choppiness metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_1w_min: Price<ComputedFromHeight<Cents, M>>,
    pub price_1w_max: Price<ComputedFromHeight<Cents, M>>,
    pub price_2w_min: Price<ComputedFromHeight<Cents, M>>,
    pub price_2w_max: Price<ComputedFromHeight<Cents, M>>,
    pub price_1m_min: Price<ComputedFromHeight<Cents, M>>,
    pub price_1m_max: Price<ComputedFromHeight<Cents, M>>,
    pub price_1y_min: Price<ComputedFromHeight<Cents, M>>,
    pub price_1y_max: Price<ComputedFromHeight<Cents, M>>,
    pub price_true_range: ComputedFromHeight<StoredF32, M>,
    pub price_true_range_2w_sum: ComputedFromHeight<StoredF32, M>,
    pub price_2w_choppiness_index: ComputedFromHeight<StoredF32, M>,
}
