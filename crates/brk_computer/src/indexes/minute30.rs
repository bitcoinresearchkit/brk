use brk_traversable::Traversable;
use brk_types::{Height, Minute30, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Minute30, Minute30>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Minute30, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "minute30", version)?,
            first_height: EagerVec::forced_import(db, "minute30_first_height", version)?,
        })
    }
}
