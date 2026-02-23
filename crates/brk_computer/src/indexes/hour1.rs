use brk_traversable::Traversable;
use brk_types::{Height, Hour1, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Hour1, Hour1>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Hour1, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "hour1", version)?,
            first_height: EagerVec::forced_import(db, "hour1_first_height", version)?,
        })
    }
}
