use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedDateLast;

/// Price range and choppiness metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_1w_min: ComputedDateLast<Dollars>,
    pub price_1w_max: ComputedDateLast<Dollars>,
    pub price_2w_min: ComputedDateLast<Dollars>,
    pub price_2w_max: ComputedDateLast<Dollars>,
    pub price_1m_min: ComputedDateLast<Dollars>,
    pub price_1m_max: ComputedDateLast<Dollars>,
    pub price_1y_min: ComputedDateLast<Dollars>,
    pub price_1y_max: ComputedDateLast<Dollars>,
    pub dateindex_to_price_true_range: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_price_true_range_2w_sum: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub price_2w_choppiness_index: ComputedDateLast<StoredF32>,
}
