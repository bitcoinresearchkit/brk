use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightCumulativeSum};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_value_destroyed: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
            )?,
            cointime_value_created: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
            )?,
            cointime_value_stored: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
            )?,
            vocdd: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "vocdd",
                version + Version::ONE,
                indexes,
            )?,
        })
    }
}
