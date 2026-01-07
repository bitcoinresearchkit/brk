//! Derived date periods with average-value aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyAverage};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateAverage<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyAverage<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyAverage<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyAverage<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyAverage<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyAverage<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyAverage<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateAverage<T>
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
            weekindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyAverage::from_source(
                name,
                version + VERSION,
                dateindex_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
