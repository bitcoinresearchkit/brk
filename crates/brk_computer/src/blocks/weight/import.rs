use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, ComputedHeightDerivedCumulativeFull, RollingDistribution},
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
            ComputedFromHeightLast::forced_import(db, "block_fullness", version, indexes)?;

        let fullness_rolling =
            RollingDistribution::forced_import(db, "block_fullness", version, indexes)?;

        Ok(Self {
            weight,
            fullness,
            fullness_rolling,
        })
    }
}
