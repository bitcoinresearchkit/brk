use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, StoredF32, StoredF64, StoredU32};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSum, ComputedFromDateLast, ComputedHeightDerivedLast,
};

/// Difficulty metrics: raw difficulty, derived stats, adjustment, and countdown
#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Raw difficulty with dateindex/period stats - merges with indexer's raw
    pub raw: ComputedHeightDerivedLast<StoredF64>,
    pub as_hash: ComputedFromHeightLast<StoredF32>,
    pub adjustment: ComputedFromHeightSum<StoredF32>,
    pub epoch: ComputedFromDateLast<DifficultyEpoch>,
    pub blocks_before_next_adjustment: ComputedFromHeightLast<StoredU32>,
    pub days_before_next_adjustment: ComputedFromHeightLast<StoredF32>,
}
