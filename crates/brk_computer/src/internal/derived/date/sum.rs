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
pub struct LazyPeriodsSum<T>
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

impl<T> LazyPeriodsSum<T>
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
                LazySum::from_source_raw(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
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
