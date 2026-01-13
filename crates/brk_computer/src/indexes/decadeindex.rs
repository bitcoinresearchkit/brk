use brk_traversable::Traversable;
use brk_types::{Date, DecadeIndex, StoredU64, Version, YearIndex};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<DecadeIndex, DecadeIndex>>,
    pub date: EagerVec<PcoVec<DecadeIndex, Date>>,
    pub first_yearindex: EagerVec<PcoVec<DecadeIndex, YearIndex>>,
    pub yearindex_count: EagerVec<PcoVec<DecadeIndex, StoredU64>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "decadeindex", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_yearindex: EagerVec::forced_import(db, "first_yearindex", version)?,
            yearindex_count: EagerVec::forced_import(db, "yearindex_count", version)?,
        })
    }
}
