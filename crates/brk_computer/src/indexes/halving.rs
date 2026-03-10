use brk_traversable::Traversable;
use brk_types::{Halving, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Halving, Halving>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Halving, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "halving", version)?,
            first_height: EagerVec::forced_import(db, "first_height", version)?,
        })
    }
}
