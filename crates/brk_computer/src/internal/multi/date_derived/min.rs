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
pub struct LazyDateDerivedMin<T>
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

impl<T> LazyDateDerivedMin<T>
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
        Self::from_source_inner(name, version, dateindex_source, indexes, false)
    }

    /// Create from an external dateindex source without adding _min suffix.
    pub fn from_source_raw(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        Self::from_source_inner(name, version, dateindex_source, indexes, true)
    }

    fn from_source_inner(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
        raw: bool,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                if raw {
                    LazyMin::from_source_raw(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
                } else {
                    LazyMin::from_source(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
                }
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
