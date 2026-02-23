use brk_traversable::Traversable;
use brk_types::{Height, Minute1, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Minute1, Minute1>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Minute1, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "minute1", version)?,
            first_height: EagerVec::forced_import(db, "minute1_first_height", version)?,
        })
    }
}
