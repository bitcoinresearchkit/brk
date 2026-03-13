use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlockFull, ResolutionsFull},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: ComputedPerBlockFull::forced_import(
                db,
                "block_vbytes",
                version,
                indexes,
                cached_starts,
            )?,
            size: ResolutionsFull::forced_import(
                db,
                "block_size",
                version,
                indexes,
                cached_starts,
            )?,
        })
    }
}
