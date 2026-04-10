use brk_traversable::Traversable;
use brk_types::{Date, Day1, Height, Version};
use vecdb::{CachedVec, Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Day1, Day1>>>,
    pub date: M::Stored<EagerVec<PcoVec<Day1, Date>>>,
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<Day1, Height>>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "day1_index", version)?,
            date: EagerVec::forced_import(db, "date", version + Version::ONE)?,
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}
