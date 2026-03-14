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

        let as_hash = LazyPerBlock::from_height_source::<DifficultyToHashF64>(
            "difficulty_as_hash",
            version,
            indexer.vecs.blocks.difficulty.read_only_boxed_clone(),
            indexes,
        );

        let blocks_before_next = PerBlock::forced_import(
            db,
            "blocks_before_next_difficulty_adjustment",
            version + v2,
            indexes,
        )?;

        let days_before_next = LazyPerBlock::from_computed::<BlocksToDaysF32>(
            "days_before_next_difficulty_adjustment",
            version + v2,
            blocks_before_next.height.read_only_boxed_clone(),
            &blocks_before_next,
        );

        Ok(Self {
            value: Resolutions::forced_import(
                "difficulty",
                indexer.vecs.blocks.difficulty.read_only_boxed_clone(),
                version,
                indexes,
            ),
            as_hash,
            adjustment: PercentPerBlock::forced_import(
                db,
                "difficulty_adjustment",
                version,
                indexes,
            )?,
            epoch: PerBlock::forced_import(db, "difficulty_epoch", version, indexes)?,
            blocks_before_next,
            days_before_next,
        })
    }
}
