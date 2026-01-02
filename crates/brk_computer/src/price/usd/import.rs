use brk_error::Result;
use brk_types::{DateIndex, Height, OHLCDollars, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1};

use super::super::ohlc;
use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        ohlc: &ohlc::Vecs,
    ) -> Result<Self> {
        let first = || VecBuilderOptions::default().add_first();
        let last = || VecBuilderOptions::default().add_last();
        let min = || VecBuilderOptions::default().add_min();
        let max = || VecBuilderOptions::default().add_max();

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
            timeindexes_to_price_open: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_open",
                Source::Compute,
                version,
                indexes,
                first(),
            )?,
            timeindexes_to_price_high: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_high",
                Source::Compute,
                version,
                indexes,
                max(),
            )?,
            timeindexes_to_price_low: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_low",
                Source::Compute,
                version,
                indexes,
                min(),
            )?,
            timeindexes_to_price_close: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_close",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
            chainindexes_to_price_open: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_open",
                version,
                indexes,
                first(),
            )?,
            chainindexes_to_price_high: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_high",
                version,
                indexes,
                max(),
            )?,
            chainindexes_to_price_low: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_low",
                version,
                indexes,
                min(),
            )?,
            chainindexes_to_price_close: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_close",
                version,
                indexes,
                last(),
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
