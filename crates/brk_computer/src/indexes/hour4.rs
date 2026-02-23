use brk_traversable::Traversable;
use brk_types::{Height, Hour4, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Hour4, Hour4>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Hour4, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "hour4", version)?,
            first_height: EagerVec::forced_import(db, "hour4_first_height", version)?,
        })
    }
}
