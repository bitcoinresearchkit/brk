use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, ComputedPerBlockCumulativeSum};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinblocks_created: ComputedPerBlockCumulativeSum<StoredF64, M>,
    pub coinblocks_stored: ComputedPerBlockCumulativeSum<StoredF64, M>,
    pub liveliness: ComputedPerBlock<StoredF64, M>,
    pub vaultedness: ComputedPerBlock<StoredF64, M>,
    pub activity_to_vaultedness_ratio: ComputedPerBlock<StoredF64, M>,
}
