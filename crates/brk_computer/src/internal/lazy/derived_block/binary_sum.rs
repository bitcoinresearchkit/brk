//! Lazy aggregated binary transform for Sum-only pattern across all time periods.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{ComputedVecValue, ComputedDerivedBlockSum, LazyBinaryDateSum, LazyBinaryTransformSum, NumericValue};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryDerivedBlockSum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyBinaryDateSum<T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformSum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryDerivedBlockSum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedDerivedBlockSum<S1T>,
        source2: &ComputedDerivedBlockSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryDateSum::from_derived::<F>(name, v, source1, source2),
            difficultyepoch: LazyBinaryTransformSum::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
