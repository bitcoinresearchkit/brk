use brk_traversable::Traversable;
use brk_types::{Day3, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Day3, Day3>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<Day3, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "day3", version)?,
            first_height: EagerVec::forced_import(db, "day3_first_height", version)?,
        })
    }
}
