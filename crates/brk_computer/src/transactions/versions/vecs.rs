use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub v2: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub v3: ComputedPerBlockCumulativeSum<StoredU64, M>,
}
