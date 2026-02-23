//! Lazy aggregated Sum for block-level sources.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedHeightDerivedSum, ComputedVecValue, LazyTransformSum, NumericValue,
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyHeightDerivedSum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub minute1: LazyTransformSum<Minute1, T, S1T>,
    pub minute5: LazyTransformSum<Minute5, T, S1T>,
    pub minute10: LazyTransformSum<Minute10, T, S1T>,
    pub minute30: LazyTransformSum<Minute30, T, S1T>,
    pub hour1: LazyTransformSum<Hour1, T, S1T>,
    pub hour4: LazyTransformSum<Hour4, T, S1T>,
    pub hour12: LazyTransformSum<Hour12, T, S1T>,
    pub day1: LazyTransformSum<Day1, T, S1T>,
    pub day3: LazyTransformSum<Day3, T, S1T>,
    pub week1: LazyTransformSum<Week1, T, S1T>,
    pub month1: LazyTransformSum<Month1, T, S1T>,
    pub month3: LazyTransformSum<Month3, T, S1T>,
    pub month6: LazyTransformSum<Month6, T, S1T>,
    pub year1: LazyTransformSum<Year1, T, S1T>,
    pub year10: LazyTransformSum<Year10, T, S1T>,
    pub halvingepoch: LazyTransformSum<HalvingEpoch, T, S1T>,
    pub difficultyepoch: LazyTransformSum<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedSum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedHeightDerivedSum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformSum::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
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
