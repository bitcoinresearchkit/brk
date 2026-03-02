use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, StoredF32, StoredF64, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, ComputedHeightDerived};

/// Difficulty metrics: raw difficulty, derived stats, adjustment, and countdown
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub raw: ComputedHeightDerived<StoredF64>,
    pub as_hash: ComputedFromHeight<StoredF64, M>,
    pub adjustment: ComputedFromHeight<StoredF32, M>,
    pub epoch: ComputedFromHeight<DifficultyEpoch, M>,
    pub blocks_before_next_adjustment: ComputedFromHeight<StoredU32, M>,
    pub days_before_next_adjustment: ComputedFromHeight<StoredF32, M>,
}
