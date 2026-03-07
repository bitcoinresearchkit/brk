use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedHeightDerivedFull, PercentFromHeightRollingAverage},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let weight =
            ComputedHeightDerivedFull::forced_import(db, "block_weight", version, indexes)?;

        let fullness = PercentFromHeightRollingAverage::forced_import(
            db,
            "block_fullness",
            version,
            indexes,
        )?;

        Ok(Self { weight, fullness })
    }
}
