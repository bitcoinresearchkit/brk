use brk_traversable::Traversable;
use brk_types::StoredF64;

use crate::internal::{ComputedFromHeightLast, ComputedFromHeightSumCum};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub coinblocks_created: ComputedFromHeightSumCum<StoredF64>,
    pub coinblocks_stored: ComputedFromHeightSumCum<StoredF64>,
    pub liveliness: ComputedFromHeightLast<StoredF64>,
    pub vaultedness: ComputedFromHeightLast<StoredF64>,
    pub activity_to_vaultedness_ratio: ComputedFromHeightLast<StoredF64>,
}
