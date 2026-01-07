//! Derived date periods with last-value aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyLast};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyLast<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyLast<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyLast<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyLast<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyLast<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyLast<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateLast<T>
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
            weekindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source.clone(),
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyLast::from_source(
                name,
                version + VERSION,
                dateindex_source,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}
