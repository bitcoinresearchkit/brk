use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<DifficultyEpoch, DifficultyEpoch>>,
    pub first_height: EagerVec<PcoVec<DifficultyEpoch, Height>>,
    pub height_count: EagerVec<PcoVec<DifficultyEpoch, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "difficultyepoch", version)?,
            first_height: EagerVec::forced_import(db, "first_height", version)?,
            height_count: EagerVec::forced_import(db, "height_count", version)?,
        })
    }
}
