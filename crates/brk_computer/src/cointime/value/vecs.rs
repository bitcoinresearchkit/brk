use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub destroyed: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub created: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub stored: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    pub vocdd: ComputedPerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
}
