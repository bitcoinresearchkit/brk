use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

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
    ) -> Result<Self> {
        let first = || VecBuilderOptions::default().add_first();
        let last = || VecBuilderOptions::default().add_last();
        let min = || VecBuilderOptions::default().add_min();
        let max = || VecBuilderOptions::default().add_max();

        Ok(Self {
            dateindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            height_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            timeindexes_to_price_open_in_sats: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_open_in_sats",
                Source::Compute,
                version,
                indexes,
                first(),
            )?,
            timeindexes_to_price_high_in_sats: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_high_in_sats",
                Source::Compute,
                version,
                indexes,
                max(),
            )?,
            timeindexes_to_price_low_in_sats: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_low_in_sats",
                Source::Compute,
                version,
                indexes,
                min(),
            )?,
            timeindexes_to_price_close_in_sats: ComputedVecsFromDateIndex::forced_import(
                db,
                "price_close_in_sats",
                Source::Compute,
                version,
                indexes,
                last(),
            )?,
            chainindexes_to_price_open_in_sats: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_open_in_sats",
                version,
                indexes,
                first(),
            )?,
            chainindexes_to_price_high_in_sats: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_high_in_sats",
                version,
                indexes,
                max(),
            )?,
            chainindexes_to_price_low_in_sats: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_low_in_sats",
                version,
                indexes,
                min(),
            )?,
            chainindexes_to_price_close_in_sats: ComputedVecsFromHeightStrict::forced_import(
                db,
                "price_close_in_sats",
                version,
                indexes,
                last(),
            )?,
            weekindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            difficultyepoch_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            monthindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            quarterindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            semesterindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            yearindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
            decadeindex_to_price_ohlc_in_sats: EagerVec::forced_import(
                db,
                "price_ohlc_in_sats",
                version,
            )?,
        })
    }
}
