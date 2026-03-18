use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    blocks::SizeVecs,
    indexes,
    internal::{CachedWindowStarts, LazyPerBlockRolling, PercentVec, VBytesToWeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
        size: &SizeVecs,
    ) -> Result<Self> {
        let weight = LazyPerBlockRolling::from_per_block_full::<VBytesToWeight>(
            "block_weight",
            version,
            &size.vbytes,
            cached_starts,
            indexes,
        );

        let fullness = PercentVec::forced_import(db, "block_fullness", version)?;

        Ok(Self { weight, fullness })
    }
}
