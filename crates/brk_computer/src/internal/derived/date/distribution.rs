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
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazyDistribution::from_distribution(
                    name, v, average_source.clone(), min_source.clone(), max_source.clone(),
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
