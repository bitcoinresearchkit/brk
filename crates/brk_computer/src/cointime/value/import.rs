use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlockCumulativeWithSums},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            destroyed: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
                cached_starts,
            )?,
            created: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
                cached_starts,
            )?,
            stored: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
                cached_starts,
            )?,
            vocdd: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "vocdd",
                version + Version::ONE,
                indexes,
                cached_starts,
            )?,
        })
    }
}
