use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedBlockLast, ComputedDateLast},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v2 = Version::TWO;

        Ok(Self {
            epoch: ComputedDateLast::forced_import(db, "difficultyepoch", version, indexes)?,
            blocks_before_next_difficulty_adjustment: ComputedBlockLast::forced_import(
                db,
                "blocks_before_next_difficulty_adjustment",
                version + v2,
                indexes,
            )?,
            days_before_next_difficulty_adjustment: ComputedBlockLast::forced_import(
                db,
                "days_before_next_difficulty_adjustment",
                version + v2,
                indexes,
            )?,
        })
    }
}
