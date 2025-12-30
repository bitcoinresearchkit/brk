use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    grouped::{ComputedVecsFromDateIndex, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = || VecBuilderOptions::default().add_last();

        Ok(Self {
            difficultyepoch_to_timestamp: EagerVec::forced_import(db, "timestamp", version + v0)?,
            halvingepoch_to_timestamp: EagerVec::forced_import(db, "timestamp", version + v0)?,
            timeindexes_to_timestamp: ComputedVecsFromDateIndex::forced_import(
                db,
                "timestamp",
                Source::Compute,
                version + v0,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            indexes_to_difficultyepoch: ComputedVecsFromDateIndex::forced_import(
                db,
                "difficultyepoch",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_halvingepoch: ComputedVecsFromDateIndex::forced_import(
                db,
                "halvingepoch",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
        })
    }
}
