use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightCumSum};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            cointime_value_destroyed: ComputedFromHeightCumSum::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
            )?,
            cointime_value_created: ComputedFromHeightCumSum::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
            )?,
            cointime_value_stored: ComputedFromHeightCumSum::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
            )?,
            vocdd: ComputedFromHeightCumSum::forced_import(
                db,
                "vocdd",
                version + Version::ONE,
                indexes,
            )?,
        })
    }
}
