use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, StoredF32, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightLast;

/// Halving epoch metrics and countdown
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub epoch: ComputedFromHeightLast<HalvingEpoch, M>,
    pub blocks_before_next_halving: ComputedFromHeightLast<StoredU32, M>,
    pub days_before_next_halving: ComputedFromHeightLast<StoredF32, M>,
}
