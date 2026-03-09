use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedPerBlockCumulativeSum};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            v1: ComputedPerBlockCumulativeSum::forced_import(db, "tx_v1", version, indexes)?,
            v2: ComputedPerBlockCumulativeSum::forced_import(db, "tx_v2", version, indexes)?,
            v3: ComputedPerBlockCumulativeSum::forced_import(db, "tx_v3", version, indexes)?,
        })
    }
}
