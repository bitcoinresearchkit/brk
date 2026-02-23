use brk_traversable::Traversable;
use brk_types::{Date, Height, Month1, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Month1, Month1>>>,
    pub date: M::Stored<EagerVec<PcoVec<Month1, Date>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Month1, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "month1", version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: EagerVec::forced_import(db, "month1_first_height", version)?,
        })
    }
}
