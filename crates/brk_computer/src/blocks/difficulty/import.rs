use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, ComputedFromHeightSum, ComputedFromDateLast,
        ComputedHeightDerivedLast,
    },
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        Ok(Self {
            raw: ComputedHeightDerivedLast::forced_import(
                db,
                "difficulty",
                indexer.vecs.blocks.difficulty.boxed_clone(),
                version,
                indexes,
            )?,
            as_hash: ComputedFromHeightLast::forced_import(db, "difficulty_as_hash", version, indexes)?,
            adjustment: ComputedFromHeightSum::forced_import(db, "difficulty_adjustment", version, indexes)?,
            epoch: ComputedFromDateLast::forced_import(db, "difficultyepoch", version, indexes)?,
            blocks_before_next_adjustment: ComputedFromHeightLast::forced_import(
                db,
                "blocks_before_next_difficulty_adjustment",
                version + v2,
                indexes,
            )?,
            days_before_next_adjustment: ComputedFromHeightLast::forced_import(
                db,
                "days_before_next_difficulty_adjustment",
                version + v2,
                indexes,
            )?,
        })
    }
}
