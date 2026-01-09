use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedBlockSumCum};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_value_destroyed: ComputedBlockSumCum::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
            )?,
            cointime_value_created: ComputedBlockSumCum::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
            )?,
            cointime_value_stored: ComputedBlockSumCum::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
            )?,
        })
    }
}
