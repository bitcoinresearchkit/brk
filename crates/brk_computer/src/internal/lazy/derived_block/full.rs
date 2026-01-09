//! Lazy aggregated Full for block-level sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedVecValue, ComputedDerivedBlockFull, LazyPeriodsFull, Full, LazyDateFull, NumericValue,
};

use super::super::transform::LazyTransformStats;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlockFull<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyDateFull<T, S1T>,
    pub difficultyepoch: LazyTransformStats<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyDerivedBlockFull<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &Full<DateIndex, S1T>,
        periods: &LazyPeriodsFull<S1T>,
        difficultyepoch: &crate::internal::LazyFull<
            DifficultyEpoch,
            S1T,
            brk_types::Height,
            DifficultyEpoch,
        >,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDateFull::from_full::<F>(name, v, dateindex, periods),
            difficultyepoch: LazyTransformStats::from_boxed::<F>(
                name,
                v,
                difficultyepoch.average.boxed_clone(),
                difficultyepoch.min.boxed_clone(),
                difficultyepoch.max.boxed_clone(),
                difficultyepoch.sum.boxed_clone(),
                difficultyepoch.cumulative.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedDerivedBlockFull<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDateFull::from_full::<F>(name, v, &source.dateindex, &source.dates),
            difficultyepoch: LazyTransformStats::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.average.boxed_clone(),
                source.difficultyepoch.min.boxed_clone(),
                source.difficultyepoch.max.boxed_clone(),
                source.difficultyepoch.sum.boxed_clone(),
                source.difficultyepoch.cumulative.boxed_clone(),
            ),
        }
    }
}
