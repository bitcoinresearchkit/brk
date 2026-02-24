use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightCumSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub cointime_value_destroyed: ComputedFromHeightCumSum<StoredF64, M>,
    pub cointime_value_created: ComputedFromHeightCumSum<StoredF64, M>,
    pub cointime_value_stored: ComputedFromHeightCumSum<StoredF64, M>,
    pub vocdd: ComputedFromHeightCumSum<StoredF64, M>,
}
