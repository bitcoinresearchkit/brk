//! Lazy transform for Sum-only date sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, LazyPeriodsSum, LazyTransformSum};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDateSum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: LazyTransformSum<DateIndex, T, S1T>,
    pub weekindex: LazyTransformSum<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformSum<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformSum<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformSum<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformSum<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformSum<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyDateSum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, S1T>,
        source: &LazyPeriodsSum<S1T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformSum::from_boxed::<F>(name, v, source.$p.boxed_clone())
            };
        }

        Self {
            dateindex: LazyTransformSum::from_boxed::<F>(name, v, dateindex_source),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
