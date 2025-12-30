use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, StoredF32};
use vecdb::{EagerVec, PcoVec};

use crate::grouped::ComputedVecsFromDateIndex;

/// Price range and choppiness metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_price_1w_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1w_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_2w_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_2w_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1m_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1m_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1y_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1y_max: ComputedVecsFromDateIndex<Dollars>,
    pub dateindex_to_price_true_range: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_price_true_range_2w_sum: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_price_2w_choppiness_index: ComputedVecsFromDateIndex<StoredF32>,
}
