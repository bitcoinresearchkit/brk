use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, Height, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex,
    YearIndex,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecValue};

use crate::indexes;

/// Lazy constant vecs for all index levels.
/// Uses const generic transforms to return the same value for every index.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ConstantVecs<T>
where
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    pub height: LazyVecFrom1<Height, T, Height, Height>,
    pub dateindex: LazyVecFrom1<DateIndex, T, DateIndex, DateIndex>,
    pub weekindex: LazyVecFrom1<WeekIndex, T, WeekIndex, WeekIndex>,
    pub monthindex: LazyVecFrom1<MonthIndex, T, MonthIndex, MonthIndex>,
    pub quarterindex: LazyVecFrom1<QuarterIndex, T, QuarterIndex, QuarterIndex>,
    pub semesterindex: LazyVecFrom1<SemesterIndex, T, SemesterIndex, SemesterIndex>,
    pub yearindex: LazyVecFrom1<YearIndex, T, YearIndex, YearIndex>,
    pub decadeindex: LazyVecFrom1<DecadeIndex, T, DecadeIndex, DecadeIndex>,
}

impl<T: VecValue + Formattable + Serialize + JsonSchema> ConstantVecs<T> {
    /// Create constant vecs using a transform that ignores input and returns a constant.
    pub fn new<F>(name: &str, version: Version, indexes: &indexes::Vecs) -> Self
    where
        F: UnaryTransform<Height, T>
            + UnaryTransform<DateIndex, T>
            + UnaryTransform<WeekIndex, T>
            + UnaryTransform<MonthIndex, T>
            + UnaryTransform<QuarterIndex, T>
            + UnaryTransform<SemesterIndex, T>
            + UnaryTransform<YearIndex, T>
            + UnaryTransform<DecadeIndex, T>,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.height.identity.boxed_clone(),
            ),
            dateindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.dateindex.identity.boxed_clone(),
            ),
            weekindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.weekindex.identity.boxed_clone(),
            ),
            monthindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.monthindex.identity.boxed_clone(),
            ),
            quarterindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.quarterindex.identity.boxed_clone(),
            ),
            semesterindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.semesterindex.identity.boxed_clone(),
            ),
            yearindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.yearindex.identity.boxed_clone(),
            ),
            decadeindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.decadeindex.identity.boxed_clone(),
            ),
        }
    }
}

