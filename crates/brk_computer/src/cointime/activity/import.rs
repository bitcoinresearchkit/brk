use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlock, ComputedPerBlockCumulativeWithSums},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        Ok(Self {
            coinblocks_created: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "coinblocks_created",
                version,
                indexes,
                cached_starts,
            )?,
            coinblocks_stored: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "coinblocks_stored",
                version,
                indexes,
                cached_starts,
            )?,
            liveliness: ComputedPerBlock::forced_import(db, "liveliness", version, indexes)?,
            vaultedness: ComputedPerBlock::forced_import(db, "vaultedness", version, indexes)?,
            ratio: ComputedPerBlock::forced_import(
                db,
                "activity_to_vaultedness_ratio",
                version,
                indexes,
            )?,
        })
    }
}
