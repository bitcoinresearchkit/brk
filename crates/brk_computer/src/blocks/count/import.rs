use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{BlockCountTarget, CachedWindowStarts, ComputedPerBlockCumulativeWithSums, ConstantVecs},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            target: ConstantVecs::new::<BlockCountTarget>(
                "block_count_target",
                version,
                indexes,
            ),
            total: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "block_count",
                version + Version::ONE,
                indexes,
                cached_starts,
            )?,
        })
    }
}
