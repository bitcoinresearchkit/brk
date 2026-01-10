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
pub struct LazyPeriodsFirst<T>
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

impl<T> LazyPeriodsFirst<T>
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

        macro_rules! period {
            ($idx:ident) => {
                LazyFirst::from_source(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
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
