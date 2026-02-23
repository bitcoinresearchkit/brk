//! ComputedHeightDerivedDistribution - lazy time periods + epochs.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{ComputedVecValue, LazyDistribution, NumericValue},
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedDistribution<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub minute1: LazyDistribution<Minute1, T, Height, Height>,
    pub minute5: LazyDistribution<Minute5, T, Height, Height>,
    pub minute10: LazyDistribution<Minute10, T, Height, Height>,
    pub minute30: LazyDistribution<Minute30, T, Height, Height>,
    pub hour1: LazyDistribution<Hour1, T, Height, Height>,
    pub hour4: LazyDistribution<Hour4, T, Height, Height>,
    pub hour12: LazyDistribution<Hour12, T, Height, Height>,
    pub day1: LazyDistribution<Day1, T, Height, Height>,
    pub day3: LazyDistribution<Day3, T, Height, Height>,
    pub week1: LazyDistribution<Week1, T, Height, Height>,
    pub month1: LazyDistribution<Month1, T, Height, Height>,
    pub month3: LazyDistribution<Month3, T, Height, Height>,
    pub month6: LazyDistribution<Month6, T, Height, Height>,
    pub year1: LazyDistribution<Year1, T, Height, Height>,
    pub year10: LazyDistribution<Year10, T, Height, Height>,
    pub halvingepoch: LazyDistribution<HalvingEpoch, T, Height, HalvingEpoch>,
    pub difficultyepoch: LazyDistribution<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedDistribution<T>
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
                LazyDistribution::from_height_source(
                    name,
                    v,
                    height_source.clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazyDistribution::from_source(
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
