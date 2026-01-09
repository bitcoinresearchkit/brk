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

        macro_rules! period {
            ($p:ident) => {
                LazyTransformSumCum::from_boxed_sum_raw::<F>(
                    name, v, source.$p.sum.boxed_clone(), source.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransformSumCum::from_sum_cum_sum_raw::<F>(name, v, dateindex),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
