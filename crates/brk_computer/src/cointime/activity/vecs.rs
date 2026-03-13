use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, ComputedPerBlockCumulativeWithSums};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinblocks_created: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub coinblocks_stored: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub liveliness: ComputedPerBlock<StoredF64, M>,
    pub vaultedness: ComputedPerBlock<StoredF64, M>,
    pub ratio: ComputedPerBlock<StoredF64, M>,
}
