use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_date: EagerVec::forced_import(db, "date", version)?,
            height_to_date_fixed: EagerVec::forced_import(db, "date_fixed", version)?,
            height_to_dateindex: EagerVec::forced_import(db, "dateindex", version)?,
            height_to_difficultyepoch: EagerVec::forced_import(db, "difficultyepoch", version)?,
            height_to_halvingepoch: EagerVec::forced_import(db, "halvingepoch", version)?,
            height_to_height: EagerVec::forced_import(db, "height", version)?,
            height_to_timestamp_fixed: EagerVec::forced_import(db, "timestamp_fixed", version)?,
            height_to_txindex_count: EagerVec::forced_import(db, "txindex_count", version)?,
            difficultyepoch_to_difficultyepoch: EagerVec::forced_import(db, "difficultyepoch", version)?,
            difficultyepoch_to_first_height: EagerVec::forced_import(db, "first_height", version)?,
            difficultyepoch_to_height_count: EagerVec::forced_import(db, "height_count", version)?,
            halvingepoch_to_first_height: EagerVec::forced_import(db, "first_height", version)?,
            halvingepoch_to_halvingepoch: EagerVec::forced_import(db, "halvingepoch", version)?,
        })
    }
}
