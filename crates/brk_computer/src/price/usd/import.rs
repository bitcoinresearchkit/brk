use brk_error::Result;
use brk_types::{DateIndex, Height, OHLCDollars, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::super::ohlc;
use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedChainFirst, ComputedChainLast, ComputedChainMax, ComputedChainMin,
        ComputedDateLast, ComputedVecsDateFirst, ComputedVecsDateMax, ComputedVecsDateMin,
    },
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        ohlc: &ohlc::Vecs,
    ) -> Result<Self> {
        let height_to_price_ohlc = LazyVecFrom1::init(
            "price_ohlc",
            version,
            ohlc.height_to_ohlc_in_cents.boxed_clone(),
            |height: Height, ohlc_iter| ohlc_iter.get(height).map(OHLCDollars::from),
        );

        let height_to_price_open_in_cents = LazyVecFrom1::init(
            "price_open_in_cents",
            version,
            ohlc.height_to_ohlc_in_cents.boxed_clone(),
            |height: Height, ohlc_iter| ohlc_iter.get(height).map(|o| o.open),
        );

        let height_to_price_high_in_cents = LazyVecFrom1::init(
            "price_high_in_cents",
            version,
            ohlc.height_to_ohlc_in_cents.boxed_clone(),
            |height: Height, ohlc_iter| ohlc_iter.get(height).map(|o| o.high),
        );

        let height_to_price_low_in_cents = LazyVecFrom1::init(
            "price_low_in_cents",
            version,
            ohlc.height_to_ohlc_in_cents.boxed_clone(),
            |height: Height, ohlc_iter| ohlc_iter.get(height).map(|o| o.low),
        );

        let height_to_price_close_in_cents = LazyVecFrom1::init(
            "price_close_in_cents",
            version,
            ohlc.height_to_ohlc_in_cents.boxed_clone(),
            |height: Height, ohlc_iter| ohlc_iter.get(height).map(|o| o.close),
        );

        let dateindex_to_price_open_in_cents = LazyVecFrom1::init(
            "price_open_in_cents",
            version,
            ohlc.dateindex_to_ohlc_in_cents.boxed_clone(),
            |di: DateIndex, ohlc_iter| ohlc_iter.get(di).map(|o| o.open),
        );

        let dateindex_to_price_high_in_cents = LazyVecFrom1::init(
            "price_high_in_cents",
            version,
            ohlc.dateindex_to_ohlc_in_cents.boxed_clone(),
            |di: DateIndex, ohlc_iter| ohlc_iter.get(di).map(|o| o.high),
        );

        let dateindex_to_price_low_in_cents = LazyVecFrom1::init(
            "price_low_in_cents",
            version,
            ohlc.dateindex_to_ohlc_in_cents.boxed_clone(),
            |di: DateIndex, ohlc_iter| ohlc_iter.get(di).map(|o| o.low),
        );

        let dateindex_to_price_close_in_cents = LazyVecFrom1::init(
            "price_close_in_cents",
            version,
            ohlc.dateindex_to_ohlc_in_cents.boxed_clone(),
            |di: DateIndex, ohlc_iter| ohlc_iter.get(di).map(|o| o.close),
        );

        let dateindex_to_price_ohlc = LazyVecFrom1::init(
            "price_ohlc",
            version,
            ohlc.dateindex_to_ohlc_in_cents.boxed_clone(),
            |di: DateIndex, ohlc_iter| ohlc_iter.get(di).map(OHLCDollars::from),
        );

        Ok(Self {
            dateindex_to_price_ohlc,
            dateindex_to_price_close_in_cents,
            dateindex_to_price_high_in_cents,
            dateindex_to_price_low_in_cents,
            dateindex_to_price_open_in_cents,
            height_to_price_close_in_cents,
            height_to_price_high_in_cents,
            height_to_price_low_in_cents,
            height_to_price_open_in_cents,
            timeindexes_to_price_open: ComputedVecsDateFirst::forced_import(
                db,
                "price_open",
                version,
                indexes,
            )?,
            timeindexes_to_price_high: ComputedVecsDateMax::forced_import(
                db,
                "price_high",
                version,
                indexes,
            )?,
            timeindexes_to_price_low: ComputedVecsDateMin::forced_import(
                db,
                "price_low",
                version,
                indexes,
            )?,
            timeindexes_to_price_close: ComputedDateLast::forced_import(
                db,
                "price_close",
                version,
                indexes,
            )?,
            chainindexes_to_price_open: ComputedChainFirst::forced_import(
                db,
                "price_open",
                version,
                indexes,
            )?,
            chainindexes_to_price_high: ComputedChainMax::forced_import(
                db,
                "price_high",
                version,
                indexes,
            )?,
            chainindexes_to_price_low: ComputedChainMin::forced_import(
                db,
                "price_low",
                version,
                indexes,
            )?,
            chainindexes_to_price_close: ComputedChainLast::forced_import(
                db,
                "price_close",
                version,
                indexes,
            )?,
            weekindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            difficultyepoch_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            monthindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            quarterindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            semesterindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            yearindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            decadeindex_to_price_ohlc: EagerVec::forced_import(db, "price_ohlc", version)?,
            height_to_price_ohlc,
        })
    }
}
