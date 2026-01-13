use brk_traversable::Traversable;
use brk_types::{Date, MonthIndex, SemesterIndex, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<SemesterIndex, SemesterIndex>>,
    pub date: EagerVec<PcoVec<SemesterIndex, Date>>,
    pub first_monthindex: EagerVec<PcoVec<SemesterIndex, MonthIndex>>,
    pub monthindex_count: EagerVec<PcoVec<SemesterIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "semesterindex", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_monthindex: EagerVec::forced_import(db, "first_monthindex", version)?,
            monthindex_count: EagerVec::forced_import(db, "monthindex_count", version)?,
        })
    }
}
