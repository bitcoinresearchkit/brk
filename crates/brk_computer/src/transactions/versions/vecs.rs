use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightSumCum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub v1: ComputedFromHeightSumCum<StoredU64, M>,
    pub v2: ComputedFromHeightSumCum<StoredU64, M>,
    pub v3: ComputedFromHeightSumCum<StoredU64, M>,
}
