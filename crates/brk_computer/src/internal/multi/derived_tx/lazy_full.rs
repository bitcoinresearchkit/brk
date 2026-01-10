//! Lazy transform of DerivedTxFull.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, DerivedTxFull, LazyTransformFull, LazyTransformStats};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedTxFull<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyTransformFull<Height, T, S1T>,
    pub difficultyepoch: LazyTransformStats<DifficultyEpoch, T, S1T>,
    pub dateindex: LazyTransformStats<DateIndex, T, S1T>,
    pub weekindex: LazyTransformStats<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformStats<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformStats<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformStats<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformStats<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformStats<DecadeIndex, T, S1T>,
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

        macro_rules! period {
            ($p:ident) => {
                LazyTransformStats::from_boxed::<F>(
                    name, v,
                    source.$p.average.boxed_clone(), source.$p.min.boxed_clone(),
                    source.$p.max.boxed_clone(), source.$p.sum.boxed_clone(),
                    source.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            height: LazyTransformFull::from_stats_aggregate::<F>(name, v, &source.height),
            difficultyepoch: period!(difficultyepoch),
            dateindex: LazyTransformStats::from_boxed::<F>(
                name, v,
                source.dateindex.average.0.boxed_clone(),
                source.dateindex.minmax.min.0.boxed_clone(),
                source.dateindex.minmax.max.0.boxed_clone(),
                source.dateindex.sum_cum.sum.0.boxed_clone(),
                source.dateindex.sum_cum.cumulative.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
