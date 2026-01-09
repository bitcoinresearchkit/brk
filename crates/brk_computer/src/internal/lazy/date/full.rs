//! Lazy transform for Full date sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, DerivedDateFull, Full};

use super::super::transform::{LazyTransformFull, LazyTransformStats};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDateFull<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: LazyTransformFull<DateIndex, T, S1T>,
    pub weekindex: LazyTransformStats<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformStats<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformStats<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformStats<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformStats<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformStats<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyDateFull<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_full<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &Full<DateIndex, S1T>,
        source: &DerivedDateFull<S1T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformStats::from_boxed::<F>(
                    name, v,
                    source.$p.average.boxed_clone(), source.$p.min.boxed_clone(),
                    source.$p.max.boxed_clone(), source.$p.sum.boxed_clone(),
                    source.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransformFull::from_stats_aggregate::<F>(name, v, dateindex),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
