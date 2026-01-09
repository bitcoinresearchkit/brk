use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{OHLCComputedVecs, OHLCPeriodVecs},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            split: OHLCComputedVecs::forced_import(db, "price", version, indexes)?,
            ohlc: OHLCPeriodVecs {
                dateindex: EagerVec::forced_import(db, "price_ohlc", version)?,
                week: EagerVec::forced_import(db, "price_ohlc", version)?,
                month: EagerVec::forced_import(db, "price_ohlc", version)?,
                quarter: EagerVec::forced_import(db, "price_ohlc", version)?,
                semester: EagerVec::forced_import(db, "price_ohlc", version)?,
                year: EagerVec::forced_import(db, "price_ohlc", version)?,
                decade: EagerVec::forced_import(db, "price_ohlc", version)?,
                height: EagerVec::forced_import(db, "price_ohlc", version)?,
                difficultyepoch: EagerVec::forced_import(db, "price_ohlc", version)?,
            },
        })
    }
}
