use brk_traversable::Traversable;
use brk_types::{HalvingEpoch, Height, Version};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec};

use brk_error::Result;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub identity: EagerVec<PcoVec<HalvingEpoch, HalvingEpoch>>,
    pub first_height: EagerVec<PcoVec<HalvingEpoch, Height>>,
}

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "halvingepoch", version)?,
            first_height: EagerVec::forced_import(db, "first_height", version)?,
        })
    }
}
