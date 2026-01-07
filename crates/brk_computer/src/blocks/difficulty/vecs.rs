use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, StoredF32, StoredU32};

use crate::internal::{ComputedBlockLast, ComputedDateLast};

/// Difficulty epoch metrics and countdown
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_difficultyepoch: ComputedDateLast<DifficultyEpoch>,
    pub indexes_to_blocks_before_next_difficulty_adjustment: ComputedBlockLast<StoredU32>,
    pub indexes_to_days_before_next_difficulty_adjustment: ComputedBlockLast<StoredF32>,
}
