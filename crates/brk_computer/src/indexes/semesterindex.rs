use brk_traversable::Traversable;
use brk_types::{MonthIndex, SemesterIndex, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<SemesterIndex, SemesterIndex>>,
    pub first_monthindex: EagerVec<PcoVec<SemesterIndex, MonthIndex>>,
    pub monthindex_count: EagerVec<PcoVec<SemesterIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "semesterindex", version)?,
            first_monthindex: EagerVec::forced_import(db, "semesterindex_first_monthindex", version)?,
            monthindex_count: EagerVec::forced_import(db, "semesterindex_monthindex_count", version)?,
        })
    }
}
