use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec, PcoVec};

use super::Vecs;

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            txoutindex: PcoVec::forced_import(db, "txoutindex", version)?,
            value: PcoVec::forced_import(db, "value", version)?,
        })
    }
}
