use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, HalvingEpoch, Height, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<Height, Height>>,
    pub dateindex: EagerVec<PcoVec<Height, DateIndex>>,
    pub difficultyepoch: EagerVec<PcoVec<Height, DifficultyEpoch>>,
    pub halvingepoch: EagerVec<PcoVec<Height, HalvingEpoch>>,
    pub txindex_count: EagerVec<PcoVec<Height, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "height", version)?,
            dateindex: EagerVec::forced_import(db, "dateindex", version)?,
            difficultyepoch: EagerVec::forced_import(db, "difficultyepoch", version)?,
            halvingepoch: EagerVec::forced_import(db, "halvingepoch", version)?,
            txindex_count: EagerVec::forced_import(db, "txindex_count", version)?,
        })
    }
}
