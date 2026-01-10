use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedOHLC, LazyFromHeightAndDateOHLC},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        Ok(Self {
            split: ComputedOHLC::forced_import(db, "price_sats", version, indexes)?,
            ohlc: LazyFromHeightAndDateOHLC {
                dateindex: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                week: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                month: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                quarter: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                semester: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                year: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                decade: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                height: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
                difficultyepoch: EagerVec::forced_import(db, "price_ohlc_sats", version)?,
            },
        })
    }
}
