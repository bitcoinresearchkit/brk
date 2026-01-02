use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec, PcoVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            txinindex_to_txoutindex: PcoVec::forced_import(db, "txoutindex", version)?,
            txinindex_to_value: PcoVec::forced_import(db, "value", version)?,
        })
    }
}
