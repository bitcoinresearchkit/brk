use brk_traversable::Traversable;
use brk_types::{MonthIndex, QuarterIndex, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<QuarterIndex, QuarterIndex>>,
    pub first_monthindex: EagerVec<PcoVec<QuarterIndex, MonthIndex>>,
    pub monthindex_count: EagerVec<PcoVec<QuarterIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "quarterindex", version)?,
            first_monthindex: EagerVec::forced_import(db, "quarterindex_first_monthindex", version)?,
            monthindex_count: EagerVec::forced_import(db, "quarterindex_monthindex_count", version)?,
        })
    }
}
