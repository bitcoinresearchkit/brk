use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<HalvingEpoch, HalvingEpoch>>>,
    pub first_height: M::Stored<EagerVec<PcoVec<HalvingEpoch, Height>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "halvingepoch", version)?,
            first_height: EagerVec::forced_import(db, "first_height", version)?,
        })
    }
}
