use brk_traversable::Traversable;
use brk_types::{Date, Height, Version, Year1};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Year1, Year1>>>,
    pub date: M::Stored<EagerVec<PcoVec<Year1, Date>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Year1, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "year1", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: EagerVec::forced_import(db, "year1_first_height", version)?,
        })
    }
}
