use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{CachedWindowStarts, ComputedPerBlockRollingAverage}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let interval = ComputedPerBlockRollingAverage::forced_import(
            db,
            "block_interval",
            version,
            indexes,
            cached_starts,
        )?;

        Ok(Self(interval))
    }
}
