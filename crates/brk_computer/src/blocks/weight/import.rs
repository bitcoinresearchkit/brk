use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightDistribution, ComputedHeightDerivedCumulativeFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let weight = ComputedHeightDerivedCumulativeFull::forced_import(
            db,
            "block_weight",
            version,
            indexes,
        )?;

        let fullness =
            ComputedFromHeightDistribution::forced_import(db, "block_fullness", version, indexes)?;

        Ok(Self { weight, fullness })
    }
}
