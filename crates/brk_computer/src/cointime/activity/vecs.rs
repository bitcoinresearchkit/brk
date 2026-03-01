use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightCumulativeSum, ComputedFromHeight};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinblocks_created: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub coinblocks_stored: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub liveliness: ComputedFromHeight<StoredF64, M>,
    pub vaultedness: ComputedFromHeight<StoredF64, M>,
    pub activity_to_vaultedness_ratio: ComputedFromHeight<StoredF64, M>,
}
