use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::ComputedBlockSumCum;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub cointime_value_destroyed: ComputedBlockSumCum<StoredF64>,
    pub cointime_value_created: ComputedBlockSumCum<StoredF64>,
    pub cointime_value_stored: ComputedBlockSumCum<StoredF64>,
}
