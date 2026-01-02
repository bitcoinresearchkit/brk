use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            txoutindex_to_txinindex: BytesVec::forced_import(db, "txinindex", version)?,
        })
    }
}
