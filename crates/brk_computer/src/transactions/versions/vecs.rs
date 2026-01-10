use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::ComputedFromHeightSumCum;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub v1: ComputedFromHeightSumCum<StoredU64>,
    pub v2: ComputedFromHeightSumCum<StoredU64>,
    pub v3: ComputedFromHeightSumCum<StoredU64>,
}
