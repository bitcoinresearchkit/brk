use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{CachedWindowStarts, ComputedPerBlockAggregated}};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self(ComputedPerBlockAggregated::forced_import(
            db,
            "input_count",
            version,
            indexes,
            cached_starts,
        )?))
    }
}
