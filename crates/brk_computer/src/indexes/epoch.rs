use brk_traversable::Traversable;
use brk_types::{Epoch, Height, StoredU64, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Epoch, Epoch>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Epoch, Height>>>,
    pub height_count: M::Stored<EagerVec<PcoVec<Epoch, StoredU64>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "epoch", version)?,
            first_height: EagerVec::forced_import(db, "first_height", version)?,
            height_count: EagerVec::forced_import(db, "height_count", version)?,
        })
    }
}
