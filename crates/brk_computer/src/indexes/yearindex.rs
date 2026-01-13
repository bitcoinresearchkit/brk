use brk_traversable::Traversable;
use brk_types::{Date, DecadeIndex, MonthIndex, StoredU64, Version, YearIndex};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<YearIndex, YearIndex>>,
    pub date: EagerVec<PcoVec<YearIndex, Date>>,
    pub first_monthindex: EagerVec<PcoVec<YearIndex, MonthIndex>>,
    pub monthindex_count: EagerVec<PcoVec<YearIndex, StoredU64>>,
    pub decadeindex: EagerVec<PcoVec<YearIndex, DecadeIndex>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "yearindex", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_monthindex: EagerVec::forced_import(db, "first_monthindex", version)?,
            monthindex_count: EagerVec::forced_import(db, "monthindex_count", version)?,
            decadeindex: EagerVec::forced_import(db, "decadeindex", version)?,
        })
    }
}
