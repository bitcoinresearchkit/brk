//! Derived date periods with sum+cumulative aggregation.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec};

use crate::{indexes, internal::LazySumCum};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DerivedDateSumCum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub weekindex: LazySumCum<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazySumCum<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazySumCum<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazySumCum<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazySumCum<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazySumCum<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedDateSumCum<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from external dateindex sum and cumulative sources.
    pub fn from_sources(
        name: &str,
        version: Version,
        sum_source: IterableBoxedVec<DateIndex, T>,
        cumulative_source: IterableBoxedVec<DateIndex, T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazySumCum::from_sources_sum_raw(
                    name, v, sum_source.clone(), cumulative_source.clone(),
                    indexes.$idx.identity.boxed_clone(),
                )
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
