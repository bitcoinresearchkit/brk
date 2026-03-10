use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub destroyed: ComputedPerBlockCumulativeSum<StoredF64, M>,
    pub created: ComputedPerBlockCumulativeSum<StoredF64, M>,
    pub stored: ComputedPerBlockCumulativeSum<StoredF64, M>,
    pub vocdd: ComputedPerBlockCumulativeSum<StoredF64, M>,
}
