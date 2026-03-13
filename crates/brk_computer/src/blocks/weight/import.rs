use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, ResolutionsFull, PercentPerBlockRollingAverage},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let weight =
            ResolutionsFull::forced_import(db, "block_weight", version, indexes, cached_starts)?;

        let fullness = PercentPerBlockRollingAverage::forced_import(
            db,
            "block_fullness",
            version,
            indexes,
            cached_starts,
        )?;

        Ok(Self { weight, fullness })
    }
}
