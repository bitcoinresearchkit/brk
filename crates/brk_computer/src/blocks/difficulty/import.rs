use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        BlocksToDaysF32, DifficultyToHashF64, LazyPerBlock, PerBlock, PercentPerBlock, Resolutions,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        let hashrate = LazyPerBlock::from_height_source::<DifficultyToHashF64>(
            "difficulty_hashrate",
            version,
            indexer.vecs.blocks.difficulty.read_only_boxed_clone(),
            indexes,
        );

        let blocks_to_retarget =
            PerBlock::forced_import(db, "blocks_to_retarget", version + v2, indexes)?;

        let days_to_retarget = LazyPerBlock::from_computed::<BlocksToDaysF32>(
            "days_to_retarget",
            version + v2,
            blocks_to_retarget.height.read_only_boxed_clone(),
            &blocks_to_retarget,
        );

        Ok(Self {
            value: Resolutions::forced_import(
                "difficulty",
                indexer.vecs.blocks.difficulty.read_only_boxed_clone(),
                version,
                indexes,
            ),
            hashrate,
            adjustment: PercentPerBlock::forced_import(
                db,
                "difficulty_adjustment",
                version + Version::ONE,
                indexes,
            )?,
            epoch: PerBlock::forced_import(db, "difficulty_epoch", version, indexes)?,
            blocks_to_retarget,
            days_to_retarget,
        })
    }
}
