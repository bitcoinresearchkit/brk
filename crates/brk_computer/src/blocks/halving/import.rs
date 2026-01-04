use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v2 = Version::TWO;
        let last = || VecBuilderOptions::default().add_last();

        Ok(Self {
            indexes_to_halvingepoch: ComputedVecsFromDateIndex::forced_import(
                db,
                "halvingepoch",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
            indexes_to_blocks_before_next_halving: ComputedVecsFromHeight::forced_import(
                db,
                "blocks_before_next_halving",
                Source::Compute,
                version + v2,
                indexes,
                last(),
            )?,
            indexes_to_days_before_next_halving: ComputedVecsFromHeight::forced_import(
                db,
                "days_before_next_halving",
                Source::Compute,
                version + v2,
                indexes,
                last(),
            )?,
        })
    }
}
