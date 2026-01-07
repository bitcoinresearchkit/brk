//! Lazy transform for SumCum date sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, DerivedDateSumCum, SumCum};

use super::super::transform::LazyTransformSumCum;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDateSumCum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: LazyTransformSumCum<DateIndex, T, S1T>,
    pub weekindex: LazyTransformSumCum<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformSumCum<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformSumCum<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformSumCum<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformSumCum<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformSumCum<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyDateSumCum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_sum_cum<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &SumCum<DateIndex, S1T>,
        source: &DerivedDateSumCum<S1T>,
    ) -> Self {
        let v = version + VERSION;
        Self {
            dateindex: LazyTransformSumCum::from_sum_cum::<F>(name, v, dateindex),
            weekindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.weekindex.sum.boxed_clone(),
                source.weekindex.cumulative.boxed_clone(),
            ),
            monthindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.monthindex.sum.boxed_clone(),
                source.monthindex.cumulative.boxed_clone(),
            ),
            quarterindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.quarterindex.sum.boxed_clone(),
                source.quarterindex.cumulative.boxed_clone(),
            ),
            semesterindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.semesterindex.sum.boxed_clone(),
                source.semesterindex.cumulative.boxed_clone(),
            ),
            yearindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.yearindex.sum.boxed_clone(),
                source.yearindex.cumulative.boxed_clone(),
            ),
            decadeindex: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.decadeindex.sum.boxed_clone(),
                source.decadeindex.cumulative.boxed_clone(),
            ),
        }
    }
}
