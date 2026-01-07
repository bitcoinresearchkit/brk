//! Derived date periods with distribution aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyDistribution};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateDistribution<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyDistribution<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyDistribution<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyDistribution<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyDistribution<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyDistribution<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyDistribution<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateDistribution<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from external dateindex sources for distribution stats.
    pub fn from_sources(
        name: &str,
        version: Version,
        average_source: IterableBoxedVec<DateIndex, T>,
        min_source: IterableBoxedVec<DateIndex, T>,
        max_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        Self {
            weekindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source.clone(),
                min_source.clone(),
                max_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyDistribution::from_distribution(
                name,
                version + VERSION,
                average_source,
                min_source,
                max_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
