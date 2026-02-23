use brk_traversable::Traversable;
use brk_types::{Height, Hour12, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Hour12, Hour12>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Hour12, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "hour12", version)?,
            first_height: EagerVec::forced_import(db, "hour12_first_height", version)?,
        })
    }
}
