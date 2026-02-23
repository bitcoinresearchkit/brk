use brk_traversable::Traversable;
use brk_types::{Date, Height, Month6, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Month6, Month6>>>,
    pub date: M::Stored<EagerVec<PcoVec<Month6, Date>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Month6, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "month6", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: EagerVec::forced_import(db, "month6_first_height", version)?,
        })
    }
}
