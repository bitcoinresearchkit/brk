use brk_traversable::Traversable;
use brk_types::{DateIndex, MonthIndex, QuarterIndex, SemesterIndex, StoredU64, Version, YearIndex};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<MonthIndex, MonthIndex>>,
    pub first_dateindex: EagerVec<PcoVec<MonthIndex, DateIndex>>,
    pub dateindex_count: EagerVec<PcoVec<MonthIndex, StoredU64>>,
    pub quarterindex: EagerVec<PcoVec<MonthIndex, QuarterIndex>>,
    pub semesterindex: EagerVec<PcoVec<MonthIndex, SemesterIndex>>,
    pub yearindex: EagerVec<PcoVec<MonthIndex, YearIndex>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "monthindex", version)?,
            first_dateindex: EagerVec::forced_import(db, "monthindex_first_dateindex", version)?,
            dateindex_count: EagerVec::forced_import(db, "monthindex_dateindex_count", version)?,
            quarterindex: EagerVec::forced_import(db, "monthindex_quarterindex", version)?,
            semesterindex: EagerVec::forced_import(db, "monthindex_semesterindex", version)?,
            yearindex: EagerVec::forced_import(db, "monthindex_yearindex", version)?,
        })
    }
}
