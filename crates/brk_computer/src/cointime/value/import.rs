use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightSumCum};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_value_destroyed: ComputedFromHeightSumCum::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
            )?,
            cointime_value_created: ComputedFromHeightSumCum::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
            )?,
            cointime_value_stored: ComputedFromHeightSumCum::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
            )?,
            vocdd: ComputedFromHeightSumCum::forced_import(
                db,
                "vocdd",
                version + Version::ONE,
                indexes,
            )?,
        })
    }
}
