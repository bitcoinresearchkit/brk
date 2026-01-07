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
pub struct DerivedDateFull<T>
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

impl<T> DerivedDateFull<T>
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
        Self {
            weekindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                sum_source.clone(),
                cumulative_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                sum_source.clone(),
                cumulative_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                sum_source.clone(),
                cumulative_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                sum_source.clone(),
                cumulative_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                sum_source.clone(),
                cumulative_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyFull::from_stats_aggregate(
                name,
                version + VERSION,
                average_source,
                min_source,
                max_source,
                sum_source,
                cumulative_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
