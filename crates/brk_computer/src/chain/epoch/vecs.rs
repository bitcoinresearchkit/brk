use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, HalvingEpoch, Timestamp};
use vecdb::{EagerVec, PcoVec};

use crate::grouped::ComputedVecsFromDateIndex;

/// Epoch and timestamp metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub difficultyepoch_to_timestamp: EagerVec<PcoVec<DifficultyEpoch, Timestamp>>,
    pub halvingepoch_to_timestamp: EagerVec<PcoVec<HalvingEpoch, Timestamp>>,
    pub timeindexes_to_timestamp: ComputedVecsFromDateIndex<Timestamp>,
    pub indexes_to_difficultyepoch: ComputedVecsFromDateIndex<DifficultyEpoch>,
    pub indexes_to_halvingepoch: ComputedVecsFromDateIndex<HalvingEpoch>,
}
