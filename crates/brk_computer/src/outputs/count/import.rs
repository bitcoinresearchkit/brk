use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, Full, RollingFull},
};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            total_count: Full::forced_import(db, "output_count", version)?,
            total_count_rolling: RollingFull::forced_import(db, "output_count", version, indexes)?,
            utxo_count: ComputedFromHeightLast::forced_import(db, "exact_utxo_count", version, indexes)?,
        })
    }
}
