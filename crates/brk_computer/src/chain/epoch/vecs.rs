use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, HalvingEpoch, StoredF32, StoredU32};

use crate::grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight};

/// Epoch metrics: difficulty epochs, halving epochs, and countdown to next epoch
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_difficultyepoch: ComputedVecsFromDateIndex<DifficultyEpoch>,
    pub indexes_to_halvingepoch: ComputedVecsFromDateIndex<HalvingEpoch>,
    // Countdown metrics (moved from mining)
    pub indexes_to_blocks_before_next_difficulty_adjustment: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_days_before_next_difficulty_adjustment: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_blocks_before_next_halving: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_days_before_next_halving: ComputedVecsFromHeight<StoredF32>,
}
