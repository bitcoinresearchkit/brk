use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedFromDateLast;

/// Price range and choppiness metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_min: ComputedFromDateLast<Dollars>,
    pub price_1w_max: ComputedFromDateLast<Dollars>,
    pub price_2w_min: ComputedFromDateLast<Dollars>,
    pub price_2w_max: ComputedFromDateLast<Dollars>,
    pub price_1m_min: ComputedFromDateLast<Dollars>,
    pub price_1m_max: ComputedFromDateLast<Dollars>,
    pub price_1y_min: ComputedFromDateLast<Dollars>,
    pub price_1y_max: ComputedFromDateLast<Dollars>,
    pub price_true_range: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub price_true_range_2w_sum: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub price_2w_choppiness_index: ComputedFromDateLast<StoredF32>,
}
