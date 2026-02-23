//! Lazy aggregated Distribution for block-level sources.
//! Like LazyHeightDerivedFull but without sum/cumulative (for ratio/percentage metrics).

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedHeightDerivedFull, ComputedVecValue, LazyTransformDistribution, NumericValue,
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyHeightDerivedDistribution<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub minute1: LazyTransformDistribution<Minute1, T, S1T>,
    pub minute5: LazyTransformDistribution<Minute5, T, S1T>,
    pub minute10: LazyTransformDistribution<Minute10, T, S1T>,
    pub minute30: LazyTransformDistribution<Minute30, T, S1T>,
    pub hour1: LazyTransformDistribution<Hour1, T, S1T>,
    pub hour4: LazyTransformDistribution<Hour4, T, S1T>,
    pub hour12: LazyTransformDistribution<Hour12, T, S1T>,
    pub day1: LazyTransformDistribution<Day1, T, S1T>,
    pub day3: LazyTransformDistribution<Day3, T, S1T>,
    pub week1: LazyTransformDistribution<Week1, T, S1T>,
    pub month1: LazyTransformDistribution<Month1, T, S1T>,
    pub month3: LazyTransformDistribution<Month3, T, S1T>,
    pub month6: LazyTransformDistribution<Month6, T, S1T>,
    pub year1: LazyTransformDistribution<Year1, T, S1T>,
    pub year10: LazyTransformDistribution<Year10, T, S1T>,
    pub halvingepoch: LazyTransformDistribution<HalvingEpoch, T, S1T>,
    pub difficultyepoch: LazyTransformDistribution<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedHeightDerivedFull<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformDistribution::from_boxed::<F>(
                    name,
                    v,
                    source.$p.average.read_only_boxed_clone(),
                    source.$p.min.read_only_boxed_clone(),
                    source.$p.max.read_only_boxed_clone(),
                    source.$p.percentiles.pct10.read_only_boxed_clone(),
                    source.$p.percentiles.pct25.read_only_boxed_clone(),
                    source.$p.percentiles.median.read_only_boxed_clone(),
                    source.$p.percentiles.pct75.read_only_boxed_clone(),
                    source.$p.percentiles.pct90.read_only_boxed_clone(),
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
