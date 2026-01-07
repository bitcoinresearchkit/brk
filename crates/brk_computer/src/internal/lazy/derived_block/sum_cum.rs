//! Lazy aggregated SumCum for block-level sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedVecValue, DerivedComputedBlockSumCum, DerivedDateSumCum, LazyDateSumCum, LazySumCum,
    NumericValue, SumCum,
};

use super::super::transform::LazyTransformSumCum;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlockSumCum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyDateSumCum<T, S1T>,
    pub difficultyepoch: LazyTransformSumCum<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyDerivedBlockSumCum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &SumCum<DateIndex, S1T>,
        periods: &DerivedDateSumCum<S1T>,
        difficultyepoch: &LazySumCum<DifficultyEpoch, S1T, Height, DifficultyEpoch>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDateSumCum::from_sum_cum::<F>(name, v, dateindex, periods),
            difficultyepoch: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                difficultyepoch.sum.boxed_clone(),
                difficultyepoch.cumulative.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &DerivedComputedBlockSumCum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDateSumCum::from_sum_cum::<F>(name, v, &source.dateindex, &source.dates),
            difficultyepoch: LazyTransformSumCum::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.sum.boxed_clone(),
                source.difficultyepoch.cumulative.boxed_clone(),
            ),
        }
    }
}
