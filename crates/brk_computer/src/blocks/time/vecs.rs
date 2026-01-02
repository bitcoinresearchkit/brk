use brk_traversable::Traversable;
use brk_types::{Date, DifficultyEpoch, Height, Timestamp};
use vecdb::{EagerVec, LazyVecFrom1, LazyVecFrom2, PcoVec};

use crate::internal::ComputedVecsFromDateIndex;

/// Timestamp and date metrics for blocks
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_date: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub height_to_date_fixed: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub height_to_timestamp_fixed: EagerVec<PcoVec<Height, Timestamp>>,
    pub difficultyepoch_to_timestamp:
        LazyVecFrom2<DifficultyEpoch, Timestamp, DifficultyEpoch, Height, Height, Timestamp>,
    pub timeindexes_to_timestamp: ComputedVecsFromDateIndex<Timestamp>,
}
