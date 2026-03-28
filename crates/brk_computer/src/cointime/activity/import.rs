use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        CachedWindowStarts, LazyPerBlock, OneMinusF64, PerBlock, PerBlockCumulativeRolling,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let liveliness = PerBlock::forced_import(db, "liveliness", version, indexes)?;

        let vaultedness = LazyPerBlock::from_computed::<OneMinusF64>(
            "vaultedness",
            version,
            liveliness.height.read_only_boxed_clone(),
            &liveliness,
        );

        Ok(Self {
            coinblocks_created: PerBlockCumulativeRolling::forced_import(
                db,
                "coinblocks_created",
                version,
                indexes,
                cached_starts,
            )?,
            coinblocks_stored: PerBlockCumulativeRolling::forced_import(
                db,
                "coinblocks_stored",
                version,
                indexes,
                cached_starts,
            )?,
            liveliness,
            vaultedness,
            ratio: PerBlock::forced_import(db, "activity_to_vaultedness", version, indexes)?,
        })
    }
}
