use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::{ComputedBlockLast, ComputedBlockSumCum};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub coinblocks_created: ComputedBlockSumCum<StoredF64>,
    pub coinblocks_stored: ComputedBlockSumCum<StoredF64>,
    pub liveliness: ComputedBlockLast<StoredF64>,
    pub vaultedness: ComputedBlockLast<StoredF64>,
    pub activity_to_vaultedness_ratio: ComputedBlockLast<StoredF64>,
}
