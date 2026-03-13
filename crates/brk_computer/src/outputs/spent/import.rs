use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec};

use super::Vecs;

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            txin_index: BytesVec::forced_import(db, "txin_index", version)?,
        })
    }
}
