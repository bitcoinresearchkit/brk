use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedFromHeightSumCum;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub cointime_value_destroyed: ComputedFromHeightSumCum<StoredF64>,
    pub cointime_value_created: ComputedFromHeightSumCum<StoredF64>,
    pub cointime_value_stored: ComputedFromHeightSumCum<StoredF64>,
}
