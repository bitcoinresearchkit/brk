use brk_traversable::Traversable;
use brk_types::{Date, Year10, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Year10, Year10>>>,
    pub date: M::Stored<EagerVec<PcoVec<Year10, Date>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Year10, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "year10", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: EagerVec::forced_import(db, "year10_first_height", version)?,
        })
    }
}
