use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedChainFirst, ComputedChainLast, ComputedChainMax, ComputedChainMin,
        ComputedDateLast, ComputedVecsDateFirst, ComputedVecsDateMax, ComputedVecsDateMin,
    },
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
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
            timeindexes_to_price_open_in_sats: ComputedVecsDateFirst::forced_import(
                db,
                "price_open_in_sats",
                version,
                indexes,
            )?,
            timeindexes_to_price_high_in_sats: ComputedVecsDateMax::forced_import(
                db,
                "price_high_in_sats",
                version,
                indexes,
            )?,
            timeindexes_to_price_low_in_sats: ComputedVecsDateMin::forced_import(
                db,
                "price_low_in_sats",
                version,
                indexes,
            )?,
            timeindexes_to_price_close_in_sats: ComputedDateLast::forced_import(
                db,
                "price_close_in_sats",
                version,
                indexes,
            )?,
            chainindexes_to_price_open_in_sats: ComputedChainFirst::forced_import(
                db,
                "price_open_in_sats",
                version,
                indexes,
            )?,
            chainindexes_to_price_high_in_sats: ComputedChainMax::forced_import(
                db,
                "price_high_in_sats",
                version,
                indexes,
            )?,
            chainindexes_to_price_low_in_sats: ComputedChainMin::forced_import(
                db,
                "price_low_in_sats",
                version,
                indexes,
            )?,
            chainindexes_to_price_close_in_sats: ComputedChainLast::forced_import(
                db,
                "price_close_in_sats",
                version,
                indexes,
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
