use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedFromDateLast, Price};

/// Price range and choppiness metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_min: Price,
    pub price_1w_max: Price,
    pub price_2w_min: Price,
    pub price_2w_max: Price,
    pub price_1m_min: Price,
    pub price_1m_max: Price,
    pub price_1y_min: Price,
    pub price_1y_max: Price,
    pub price_true_range: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub price_true_range_2w_sum: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub price_2w_choppiness_index: ComputedFromDateLast<StoredF32>,
}
