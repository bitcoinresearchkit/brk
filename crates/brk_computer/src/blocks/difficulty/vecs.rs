use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, StoredF32, StoredF64, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, ComputedHeightDerivedLast};

/// Difficulty metrics: raw difficulty, derived stats, adjustment, and countdown
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub raw: ComputedHeightDerivedLast<StoredF64>,
    pub as_hash: ComputedFromHeightLast<StoredF32, M>,
    pub adjustment: ComputedFromHeightLast<StoredF32, M>,
    pub epoch: ComputedFromHeightLast<DifficultyEpoch, M>,
    pub blocks_before_next_adjustment: ComputedFromHeightLast<StoredU32, M>,
    pub days_before_next_adjustment: ComputedFromHeightLast<StoredF32, M>,
}
