//! Derived date periods with first-value aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyFirst};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateFirst<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyFirst<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyFirst<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyFirst<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyFirst<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyFirst<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyFirst<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateFirst<T>
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
            weekindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyFirst::from_source(
                name,
                version + VERSION,
                dateindex_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
