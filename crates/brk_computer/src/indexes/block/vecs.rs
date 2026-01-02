use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, HalvingEpoch, Height, StoredU64};
use vecdb::{EagerVec, PcoVec};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_dateindex: EagerVec<PcoVec<Height, DateIndex>>,
    pub height_to_difficultyepoch: EagerVec<PcoVec<Height, DifficultyEpoch>>,
    pub height_to_halvingepoch: EagerVec<PcoVec<Height, HalvingEpoch>>,
    pub height_to_height: EagerVec<PcoVec<Height, Height>>,
    pub height_to_txindex_count: EagerVec<PcoVec<Height, StoredU64>>,
    pub difficultyepoch_to_difficultyepoch: EagerVec<PcoVec<DifficultyEpoch, DifficultyEpoch>>,
    pub difficultyepoch_to_first_height: EagerVec<PcoVec<DifficultyEpoch, Height>>,
    pub difficultyepoch_to_height_count: EagerVec<PcoVec<DifficultyEpoch, StoredU64>>,
    pub halvingepoch_to_first_height: EagerVec<PcoVec<HalvingEpoch, Height>>,
    pub halvingepoch_to_halvingepoch: EagerVec<PcoVec<HalvingEpoch, HalvingEpoch>>,
}
