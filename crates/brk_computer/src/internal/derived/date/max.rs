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
pub struct DerivedDateMax<T>
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

impl<T> DerivedDateMax<T>
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
                LazyMax::from_source(name, v, dateindex_source.clone(), indexes.$idx.identity.boxed_clone())
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
