use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlockCumulativeRolling, WindowStartVec, Windows},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            destroyed: PerBlockCumulativeRolling::forced_import(
                db,
                "cointime_value_destroyed",
                version,
                indexes,
                cached_starts,
            )?,
            created: PerBlockCumulativeRolling::forced_import(
                db,
                "cointime_value_created",
                version,
                indexes,
                cached_starts,
            )?,
            stored: PerBlockCumulativeRolling::forced_import(
                db,
                "cointime_value_stored",
                version,
                indexes,
                cached_starts,
            )?,
            vocdd: PerBlockCumulativeRolling::forced_import(
                db,
                "vocdd",
                version + Version::ONE,
                indexes,
                cached_starts,
            )?,
        })
    }
}
