//! Derived date periods with min-value aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyMin};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateMin<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyMin<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyMin<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyMin<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyMin<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyMin<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyMin<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateMin<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from an external dateindex source.
    pub fn from_source(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        Self {
            weekindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyMin::from_source(
                name,
                version + VERSION,
                dateindex_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
