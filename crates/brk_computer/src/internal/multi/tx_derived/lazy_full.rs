//! Lazy transform of TxDerivedFull.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedVecValue, TxDerivedFull, LazyTransformFull, LazyTransformStats};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyTxDerivedFull<T, S1T = T>
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

impl<T, S1T> LazyTxDerivedFull<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &TxDerivedFull<S1T>,
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
                source.dateindex.boxed_average(),
                source.dateindex.boxed_min(),
                source.dateindex.boxed_max(),
                source.dateindex.boxed_sum(),
                source.dateindex.boxed_cumulative(),
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
