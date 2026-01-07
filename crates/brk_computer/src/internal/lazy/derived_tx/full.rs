//! Lazy transform of DerivedTxFull.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, DerivedTxFull};

use super::super::transform::LazyTransformFull;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedTxFull<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyTransformFull<Height, T, S1T>,
    pub difficultyepoch: LazyTransformFull<DifficultyEpoch, T, S1T>,
    pub dateindex: LazyTransformFull<DateIndex, T, S1T>,
    pub weekindex: LazyTransformFull<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformFull<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformFull<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformFull<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformFull<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformFull<DecadeIndex, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyDerivedTxFull<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &DerivedTxFull<S1T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyTransformFull::from_stats_aggregate::<F>(name, v, &source.height),
            difficultyepoch: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.average.boxed_clone(),
                source.difficultyepoch.min.boxed_clone(),
                source.difficultyepoch.max.boxed_clone(),
                source.difficultyepoch.sum.boxed_clone(),
                source.difficultyepoch.cumulative.boxed_clone(),
            ),
            dateindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.dateindex.average.0.boxed_clone(),
                source.dateindex.minmax.min.0.boxed_clone(),
                source.dateindex.minmax.max.0.boxed_clone(),
                source.dateindex.sum_cum.sum.0.boxed_clone(),
                source.dateindex.sum_cum.cumulative.0.boxed_clone(),
            ),
            weekindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.weekindex.average.boxed_clone(),
                source.weekindex.min.boxed_clone(),
                source.weekindex.max.boxed_clone(),
                source.weekindex.sum.boxed_clone(),
                source.weekindex.cumulative.boxed_clone(),
            ),
            monthindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.monthindex.average.boxed_clone(),
                source.monthindex.min.boxed_clone(),
                source.monthindex.max.boxed_clone(),
                source.monthindex.sum.boxed_clone(),
                source.monthindex.cumulative.boxed_clone(),
            ),
            quarterindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.quarterindex.average.boxed_clone(),
                source.quarterindex.min.boxed_clone(),
                source.quarterindex.max.boxed_clone(),
                source.quarterindex.sum.boxed_clone(),
                source.quarterindex.cumulative.boxed_clone(),
            ),
            semesterindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.semesterindex.average.boxed_clone(),
                source.semesterindex.min.boxed_clone(),
                source.semesterindex.max.boxed_clone(),
                source.semesterindex.sum.boxed_clone(),
                source.semesterindex.cumulative.boxed_clone(),
            ),
            yearindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.yearindex.average.boxed_clone(),
                source.yearindex.min.boxed_clone(),
                source.yearindex.max.boxed_clone(),
                source.yearindex.sum.boxed_clone(),
                source.yearindex.cumulative.boxed_clone(),
            ),
            decadeindex: LazyTransformFull::from_boxed::<F>(
                name,
                v,
                source.decadeindex.average.boxed_clone(),
                source.decadeindex.min.boxed_clone(),
                source.decadeindex.max.boxed_clone(),
                source.decadeindex.sum.boxed_clone(),
                source.decadeindex.cumulative.boxed_clone(),
            ),
        }
    }
}
