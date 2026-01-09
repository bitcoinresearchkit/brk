use brk_traversable::Traversable;
use brk_types::{Date, DateIndex, Height, MonthIndex, StoredU64, Version, WeekIndex};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<DateIndex, DateIndex>>,
    pub date: EagerVec<PcoVec<DateIndex, Date>>,
    pub first_height: EagerVec<PcoVec<DateIndex, Height>>,
    pub height_count: EagerVec<PcoVec<DateIndex, StoredU64>>,
    pub weekindex: EagerVec<PcoVec<DateIndex, WeekIndex>>,
    pub monthindex: EagerVec<PcoVec<DateIndex, MonthIndex>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "dateindex", version)?,
            date: EagerVec::forced_import(db, "dateindex_date", version)?,
            first_height: EagerVec::forced_import(db, "dateindex_first_height", version)?,
            height_count: EagerVec::forced_import(db, "dateindex_height_count", version)?,
            weekindex: EagerVec::forced_import(db, "dateindex_weekindex", version)?,
            monthindex: EagerVec::forced_import(db, "dateindex_monthindex", version)?,
        })
    }
}
