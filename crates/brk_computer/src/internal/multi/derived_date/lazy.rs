//! Generic lazy vecs for all time period indexes.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{ComputeFrom1, Formattable, IterableCloneableVec, LazyVecFrom1, VecValue};

use crate::indexes;

/// Lazy vecs for all time period indexes (no height).
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyPeriodVecs<T>
where
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    pub dateindex: LazyVecFrom1<DateIndex, T, DateIndex, DateIndex>,
    pub weekindex: LazyVecFrom1<WeekIndex, T, WeekIndex, WeekIndex>,
    pub monthindex: LazyVecFrom1<MonthIndex, T, MonthIndex, MonthIndex>,
    pub quarterindex: LazyVecFrom1<QuarterIndex, T, QuarterIndex, QuarterIndex>,
    pub semesterindex: LazyVecFrom1<SemesterIndex, T, SemesterIndex, SemesterIndex>,
    pub yearindex: LazyVecFrom1<YearIndex, T, YearIndex, YearIndex>,
    pub decadeindex: LazyVecFrom1<DecadeIndex, T, DecadeIndex, DecadeIndex>,
}

impl<T: VecValue + Formattable + Serialize + JsonSchema> LazyPeriodVecs<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        dateindex_fn: ComputeFrom1<DateIndex, T, DateIndex, DateIndex>,
        weekindex_fn: ComputeFrom1<WeekIndex, T, WeekIndex, WeekIndex>,
        monthindex_fn: ComputeFrom1<MonthIndex, T, MonthIndex, MonthIndex>,
        quarterindex_fn: ComputeFrom1<QuarterIndex, T, QuarterIndex, QuarterIndex>,
        semesterindex_fn: ComputeFrom1<SemesterIndex, T, SemesterIndex, SemesterIndex>,
        yearindex_fn: ComputeFrom1<YearIndex, T, YearIndex, YearIndex>,
        decadeindex_fn: ComputeFrom1<DecadeIndex, T, DecadeIndex, DecadeIndex>,
    ) -> Self {
        Self {
            dateindex: LazyVecFrom1::init(
                name,
                version,
                indexes.dateindex.identity.boxed_clone(),
                dateindex_fn,
            ),
            weekindex: LazyVecFrom1::init(
                name,
                version,
                indexes.weekindex.identity.boxed_clone(),
                weekindex_fn,
            ),
            monthindex: LazyVecFrom1::init(
                name,
                version,
                indexes.monthindex.identity.boxed_clone(),
                monthindex_fn,
            ),
            quarterindex: LazyVecFrom1::init(
                name,
                version,
                indexes.quarterindex.identity.boxed_clone(),
                quarterindex_fn,
            ),
            semesterindex: LazyVecFrom1::init(
                name,
                version,
                indexes.semesterindex.identity.boxed_clone(),
                semesterindex_fn,
            ),
            yearindex: LazyVecFrom1::init(
                name,
                version,
                indexes.yearindex.identity.boxed_clone(),
                yearindex_fn,
            ),
            decadeindex: LazyVecFrom1::init(
                name,
                version,
                indexes.decadeindex.identity.boxed_clone(),
                decadeindex_fn,
            ),
        }
    }
}
