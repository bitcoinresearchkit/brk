use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub v2: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub v3: ComputedFromHeightCumulativeSum<StoredU64, M>,
}
