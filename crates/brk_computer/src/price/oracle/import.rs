use brk_error::Result;
use brk_types::Version;
use vecdb::{BytesVec, Database, EagerVec, ImportableVec, PcoVec};

use super::Vecs;
use crate::indexes;
use crate::internal::{ComputedOHLC, LazyFromHeightAndDateOHLC};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::new(11);

        let price_cents = PcoVec::forced_import(db, "oracle_price_cents", version)?;
        let ohlc_cents = BytesVec::forced_import(db, "oracle_ohlc_cents", version)?;

        let split = ComputedOHLC::forced_import(db, "oracle_price", version, indexes)?;

        let ohlc = LazyFromHeightAndDateOHLC {
            dateindex: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            week: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            month: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            quarter: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            semester: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            year: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            decade: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            height: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
            difficultyepoch: EagerVec::forced_import(db, "oracle_price_ohlc", version)?,
        };

        let ohlc_dollars = LazyFromHeightAndDateOHLC {
            dateindex: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            week: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            month: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            quarter: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            semester: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            year: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            decade: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            height: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
            difficultyepoch: EagerVec::forced_import(db, "oracle_ohlc_dollars", version)?,
        };

        Ok(Self {
            price_cents,
            ohlc_cents,
            split,
            ohlc,
            ohlc_dollars,
        })
    }
}
