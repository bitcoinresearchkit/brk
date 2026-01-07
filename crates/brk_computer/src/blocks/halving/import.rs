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
            indexes_to_halvingepoch: ComputedDateLast::forced_import(
                db,
                "halvingepoch",
                version,
                indexes,
            )?,
            indexes_to_blocks_before_next_halving: ComputedBlockLast::forced_import(
                db,
                "blocks_before_next_halving",
                version + v2,
                indexes,
            )?,
            indexes_to_days_before_next_halving: ComputedBlockLast::forced_import(
                db,
                "days_before_next_halving",
                version + v2,
                indexes,
            )?,
        })
    }
}
