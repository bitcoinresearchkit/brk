use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightCumSum, ComputedFromHeightLast};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinblocks_created: ComputedFromHeightCumSum<StoredF64, M>,
    pub coinblocks_stored: ComputedFromHeightCumSum<StoredF64, M>,
    pub liveliness: ComputedFromHeightLast<StoredF64, M>,
    pub vaultedness: ComputedFromHeightLast<StoredF64, M>,
    pub activity_to_vaultedness_ratio: ComputedFromHeightLast<StoredF64, M>,
}
