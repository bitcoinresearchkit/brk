use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock, PerBlockCumulativeWithSums};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub coinblocks_created: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub coinblocks_stored: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub liveliness: PerBlock<StoredF64, M>,
    pub vaultedness: LazyPerBlock<StoredF64>,
    pub ratio: PerBlock<StoredF64, M>,
}
