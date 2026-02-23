//! ComputedHeightDerivedSum - lazy time periods + epochs.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{ComputedVecValue, LazySum, NumericValue},
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedSum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub minute1: LazySum<Minute1, T, Height, Height>,
    pub minute5: LazySum<Minute5, T, Height, Height>,
    pub minute10: LazySum<Minute10, T, Height, Height>,
    pub minute30: LazySum<Minute30, T, Height, Height>,
    pub hour1: LazySum<Hour1, T, Height, Height>,
    pub hour4: LazySum<Hour4, T, Height, Height>,
    pub hour12: LazySum<Hour12, T, Height, Height>,
    pub day1: LazySum<Day1, T, Height, Height>,
    pub day3: LazySum<Day3, T, Height, Height>,
    pub week1: LazySum<Week1, T, Height, Height>,
    pub month1: LazySum<Month1, T, Height, Height>,
    pub month3: LazySum<Month3, T, Height, Height>,
    pub month6: LazySum<Month6, T, Height, Height>,
    pub year1: LazySum<Year1, T, Height, Height>,
    pub year10: LazySum<Year10, T, Height, Height>,
    pub halvingepoch: LazySum<HalvingEpoch, T, Height, HalvingEpoch>,
    pub difficultyepoch: LazySum<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedSum<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        name: &str,
        height_source: ReadableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazySum::from_height_source_raw(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazySum::from_source_raw(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.identity.read_only_boxed_clone(),
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
            halvingepoch: epoch!(halvingepoch),
            difficultyepoch: epoch!(difficultyepoch),
        }
    }
}
