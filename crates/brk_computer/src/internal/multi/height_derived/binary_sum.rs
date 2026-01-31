//! Lazy aggregated binary transform for Sum-only pattern across all time periods.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{ComputedVecValue, ComputedHeightDerivedSum, LazyBinaryFromDateSum, LazyBinaryTransformSum, NumericValue};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryHeightDerivedSum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyBinaryFromDateSum<T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformSum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryHeightDerivedSum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedSum<S1T>,
        source2: &ComputedHeightDerivedSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryFromDateSum::from_derived::<F>(name, v, source1, source2),
            difficultyepoch: LazyBinaryTransformSum::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    /// Create from two LazyBinaryHeightDerivedSum sources.
    pub fn from_binary<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryHeightDerivedSum<S1T, S1aT, S1bT>,
        source2: &LazyBinaryHeightDerivedSum<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryFromDateSum::from_binary::<F, _, _, _, _>(
                name,
                v,
                &source1.dates,
                &source2.dates,
            ),
            difficultyepoch: LazyBinaryTransformSum::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
