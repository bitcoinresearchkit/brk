//! Lazy OHLC period groupings derived from height-level data.
//!
//! Each period's OHLC is computed lazily in a single pass over the source range:
//! open = first, high = max, low = min, close = last.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{ComputedVecValue, LazyOHLC, OHLCRecord},
};

/// Lazy bundled OHLC vecs for all periods, derived from height-level data.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedOHLC<OHLC>
where
    OHLC: OHLCRecord + 'static,
{
    pub minute1: LazyOHLC<Minute1, OHLC, Height, OHLC::Inner, Height>,
    pub minute5: LazyOHLC<Minute5, OHLC, Height, OHLC::Inner, Height>,
    pub minute10: LazyOHLC<Minute10, OHLC, Height, OHLC::Inner, Height>,
    pub minute30: LazyOHLC<Minute30, OHLC, Height, OHLC::Inner, Height>,
    pub hour1: LazyOHLC<Hour1, OHLC, Height, OHLC::Inner, Height>,
    pub hour4: LazyOHLC<Hour4, OHLC, Height, OHLC::Inner, Height>,
    pub hour12: LazyOHLC<Hour12, OHLC, Height, OHLC::Inner, Height>,
    pub day1: LazyOHLC<Day1, OHLC, Height, OHLC::Inner, Height>,
    pub day3: LazyOHLC<Day3, OHLC, Height, OHLC::Inner, Height>,
    pub week1: LazyOHLC<Week1, OHLC, Height, OHLC::Inner, Height>,
    pub month1: LazyOHLC<Month1, OHLC, Height, OHLC::Inner, Height>,
    pub month3: LazyOHLC<Month3, OHLC, Height, OHLC::Inner, Height>,
    pub month6: LazyOHLC<Month6, OHLC, Height, OHLC::Inner, Height>,
    pub year1: LazyOHLC<Year1, OHLC, Height, OHLC::Inner, Height>,
    pub year10: LazyOHLC<Year10, OHLC, Height, OHLC::Inner, Height>,
    pub halvingepoch: LazyOHLC<HalvingEpoch, OHLC, Height, OHLC::Inner, Height>,
    pub difficultyepoch: LazyOHLC<DifficultyEpoch, OHLC, Height, OHLC::Inner, Height>,
}

const VERSION: Version = Version::ZERO;

impl<OHLC> ComputedHeightDerivedOHLC<OHLC>
where
    OHLC: OHLCRecord + 'static,
    OHLC::Inner: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn forced_import(
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        height_source: ReadableBoxedVec<Height, OHLC::Inner>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazyOHLC::from_height_source(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        Self {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        }
    }
}
