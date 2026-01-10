//! Derived date periods with max-value aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazyMax};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDateDerivedMax<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazyMax<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyMax<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyMax<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyMax<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyMax<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyMax<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> LazyDateDerivedMax<T>
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

    /// Create from an external dateindex source without adding _max suffix.
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
                    LazyMax::from_source_raw(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
                } else {
                    LazyMax::from_source(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
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
