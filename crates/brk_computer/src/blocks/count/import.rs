use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{
        BlockCountTarget1m, BlockCountTarget1w, BlockCountTarget1y, BlockCountTarget24h,
        CachedWindowStarts, ConstantVecs, PerBlockCumulativeRolling, Windows,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            target: Windows {
                _24h: ConstantVecs::new::<BlockCountTarget24h>(
                    "block_count_target_24h",
                    version,
                    indexes,
                ),
                _1w: ConstantVecs::new::<BlockCountTarget1w>(
                    "block_count_target_1w",
                    version,
                    indexes,
                ),
                _1m: ConstantVecs::new::<BlockCountTarget1m>(
                    "block_count_target_1m",
                    version,
                    indexes,
                ),
                _1y: ConstantVecs::new::<BlockCountTarget1y>(
                    "block_count_target_1y",
                    version,
                    indexes,
                ),
            },
            total: PerBlockCumulativeRolling::forced_import(
                db,
                "block_count",
                version + Version::ONE,
                indexes,
                cached_starts,
            )?,
        })
    }
}
