use brk_traversable::Traversable;
use brk_types::{Height, Minute10, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Minute10, Minute10>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Minute10, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "minute10", version)?,
            first_height: EagerVec::forced_import(db, "minute10_first_height", version)?,
        })
    }
}
