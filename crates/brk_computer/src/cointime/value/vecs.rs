use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightSumCum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub cointime_value_destroyed: ComputedFromHeightSumCum<StoredF64, M>,
    pub cointime_value_created: ComputedFromHeightSumCum<StoredF64, M>,
    pub cointime_value_stored: ComputedFromHeightSumCum<StoredF64, M>,
    pub vocdd: ComputedFromHeightSumCum<StoredF64, M>,
}
