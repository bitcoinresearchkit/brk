use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightSumCum};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            v1: ComputedFromHeightSumCum::forced_import(db, "tx_v1", version, indexes)?,
            v2: ComputedFromHeightSumCum::forced_import(db, "tx_v2", version, indexes)?,
            v3: ComputedFromHeightSumCum::forced_import(db, "tx_v3", version, indexes)?,
        })
    }
}
