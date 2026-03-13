use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{CachedWindowStarts, PerBlockRollingAverage}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let interval = PerBlockRollingAverage::forced_import(
            db,
            "block_interval",
            version,
            indexes,
            cached_starts,
        )?;

        Ok(Self(interval))
    }
}
