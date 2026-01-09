use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::ComputedBlockSumCum;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub v1: ComputedBlockSumCum<StoredU64>,
    pub v2: ComputedBlockSumCum<StoredU64>,
    pub v3: ComputedBlockSumCum<StoredU64>,
}
