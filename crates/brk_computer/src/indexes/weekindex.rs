use brk_traversable::Traversable;
use brk_types::{DateIndex, StoredU64, Version, WeekIndex};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<WeekIndex, WeekIndex>>,
    pub first_dateindex: EagerVec<PcoVec<WeekIndex, DateIndex>>,
    pub dateindex_count: EagerVec<PcoVec<WeekIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "weekindex", version)?,
            first_dateindex: EagerVec::forced_import(db, "weekindex_first_dateindex", version)?,
            dateindex_count: EagerVec::forced_import(db, "weekindex_dateindex_count", version)?,
        })
    }
}
