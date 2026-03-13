use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub v2: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub v3: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
}
