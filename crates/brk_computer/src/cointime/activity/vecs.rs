use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::{ComputedBlockLast, ComputedBlockSumCum};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_coinblocks_created: ComputedBlockSumCum<StoredF64>,
    pub indexes_to_coinblocks_stored: ComputedBlockSumCum<StoredF64>,
    pub indexes_to_liveliness: ComputedBlockLast<StoredF64>,
    pub indexes_to_vaultedness: ComputedBlockLast<StoredF64>,
    pub indexes_to_activity_to_vaultedness_ratio: ComputedBlockLast<StoredF64>,
}
