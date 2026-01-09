//! Derived date periods with full stats aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyFull};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyPeriodsFull<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyFull<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyFull<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyFull<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyFull<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyFull<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyFull<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> LazyPeriodsFull<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from external dateindex sources for full stats.
    #[allow(clippy::too_many_arguments)]
    pub fn from_sources(
        name: &str,
        version: Version,
        average_source: IterableBoxedVec<DateIndex, T>,
        min_source: IterableBoxedVec<DateIndex, T>,
        max_source: IterableBoxedVec<DateIndex, T>,
        sum_source: IterableBoxedVec<DateIndex, T>,
        cumulative_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazyFull::from_stats_aggregate(
                    name, v,
                    average_source.clone(), min_source.clone(), max_source.clone(),
                    sum_source.clone(), cumulative_source.clone(),
                    indexes.$idx.identity.boxed_clone(),
                )
            };
        }

        Self {
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
