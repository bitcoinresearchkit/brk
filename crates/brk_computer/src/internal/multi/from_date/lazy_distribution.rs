//! Lazy transform for Distribution date sources.
//! Like LazyFromDateFull but without sum/cumulative (for ratio/percentage metrics).

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedVecValue, Full, LazyDateDerivedFull, LazyTransformDistribution, LazyTransformSpread,
};

const VERSION: Version = Version::ZERO;

/// Distribution stats across date periods. Has average, min, max, percentiles but no sum/cumulative.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyFromDateDistribution<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: LazyTransformDistribution<DateIndex, T, S1T>,
    pub weekindex: LazyTransformSpread<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformSpread<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformSpread<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformSpread<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformSpread<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformSpread<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyFromDateDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_full<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &Full<DateIndex, S1T>,
        source: &LazyDateDerivedFull<S1T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformSpread::from_boxed::<F>(
                    name,
                    v,
                    source.$p.average.boxed_clone(),
                    source.$p.min.boxed_clone(),
                    source.$p.max.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransformDistribution::from_stats_aggregate::<F>(name, v, dateindex),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
