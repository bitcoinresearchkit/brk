use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub cointime_value_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub cointime_value_created: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub cointime_value_stored: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub vocdd: ComputedFromHeightCumulativeSum<StoredF64, M>,
}
