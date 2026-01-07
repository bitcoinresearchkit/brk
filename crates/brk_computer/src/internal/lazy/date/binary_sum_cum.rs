//! Binary transform for SumCum pattern across date periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{ComputedVecValue, DerivedDateFull, DerivedDateSumCum, SumCum};

use super::super::transform::LazyTransform2SumCum;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDate2SumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyTransform2SumCum<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyTransform2SumCum<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2SumCum<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2SumCum<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2SumCum<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2SumCum<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2SumCum<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDate2SumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<DateIndex, S1T>,
        periods1: &DerivedDateSumCum<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        periods2: &DerivedDateSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
            weekindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.weekindex.sum.boxed_clone(),
                periods2.weekindex.sum.boxed_clone(),
                periods1.weekindex.cumulative.boxed_clone(),
                periods2.weekindex.cumulative.boxed_clone(),
            ),
            monthindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.monthindex.sum.boxed_clone(),
                periods2.monthindex.sum.boxed_clone(),
                periods1.monthindex.cumulative.boxed_clone(),
                periods2.monthindex.cumulative.boxed_clone(),
            ),
            quarterindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.quarterindex.sum.boxed_clone(),
                periods2.quarterindex.sum.boxed_clone(),
                periods1.quarterindex.cumulative.boxed_clone(),
                periods2.quarterindex.cumulative.boxed_clone(),
            ),
            semesterindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.semesterindex.sum.boxed_clone(),
                periods2.semesterindex.sum.boxed_clone(),
                periods1.semesterindex.cumulative.boxed_clone(),
                periods2.semesterindex.cumulative.boxed_clone(),
            ),
            yearindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.yearindex.sum.boxed_clone(),
                periods2.yearindex.sum.boxed_clone(),
                periods1.yearindex.cumulative.boxed_clone(),
                periods2.yearindex.cumulative.boxed_clone(),
            ),
            decadeindex: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                periods1.decadeindex.sum.boxed_clone(),
                periods2.decadeindex.sum.boxed_clone(),
                periods1.decadeindex.cumulative.boxed_clone(),
                periods2.decadeindex.cumulative.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_full<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<DateIndex, S1T>,
        dates1: &DerivedDateFull<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        dates2: &DerivedDateFull<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
            weekindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.weekindex,
                &dates2.weekindex,
            ),
            monthindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.monthindex,
                &dates2.monthindex,
            ),
            quarterindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.quarterindex,
                &dates2.quarterindex,
            ),
            semesterindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.semesterindex,
                &dates2.semesterindex,
            ),
            yearindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.yearindex,
                &dates2.yearindex,
            ),
            decadeindex: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                &dates1.decadeindex,
                &dates2.decadeindex,
            ),
        }
    }
}
