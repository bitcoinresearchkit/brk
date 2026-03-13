use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlockFull, PerBlockRolling},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: PerBlockFull::forced_import(
                db,
                "block_vbytes",
                version,
                indexes,
                cached_starts,
            )?,
            size: PerBlockRolling::forced_import(
                db,
                "block_size",
                version,
                indexes,
                cached_starts,
            )?,
        })
    }
}
