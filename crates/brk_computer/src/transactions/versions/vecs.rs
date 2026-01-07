use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::ComputedBlockSumCum;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_tx_v1: ComputedBlockSumCum<StoredU64>,
    pub indexes_to_tx_v2: ComputedBlockSumCum<StoredU64>,
    pub indexes_to_tx_v3: ComputedBlockSumCum<StoredU64>,
}
