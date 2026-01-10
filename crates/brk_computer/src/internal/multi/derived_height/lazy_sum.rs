//! Lazy aggregated Sum for block-level sources.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedDerivedBlockSum, ComputedVecValue, LazyDateSum, LazyPeriodsSum, LazySum,
    LazyTransformSum, NumericValue, SumVec,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlockSum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyDateSum<T, S1T>,
    pub difficultyepoch: LazyTransformSum<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyDerivedBlockSum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex: &SumVec<DateIndex, S1T>,
        periods: &LazyPeriodsSum<S1T>,
        difficultyepoch: &LazySum<DifficultyEpoch, S1T, Height, DifficultyEpoch>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDateSum::from_derived::<F>(name, v, dateindex.0.boxed_clone(), periods),
            difficultyepoch: LazyTransformSum::from_boxed::<F>(
                name,
                v,
                difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedDerivedBlockSum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDateSum::from_derived::<F>(
                name,
                v,
                source.dateindex.0.boxed_clone(),
                &source.dates,
            ),
            difficultyepoch: LazyTransformSum::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
