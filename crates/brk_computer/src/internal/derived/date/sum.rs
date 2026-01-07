//! Derived date periods with sum aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazySum};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateSum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazySum<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazySum<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazySum<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazySum<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazySum<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazySum<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateSum<T>
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
        let v = version + VERSION;
        Self {
            weekindex: LazySum::from_source(
                name,
                v,
                dateindex_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazySum::from_source(
                name,
                v,
                dateindex_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazySum::from_source(
                name,
                v,
                dateindex_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazySum::from_source(
                name,
                v,
                dateindex_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazySum::from_source(
                name,
                v,
                dateindex_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazySum::from_source(
                name,
                v,
                dateindex_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
