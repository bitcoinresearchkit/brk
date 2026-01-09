//! Lazy aggregated binary transform for Sum-only pattern across all time periods.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{ComputedVecValue, DerivedComputedBlockSum, LazyDate2Sum, LazyTransform2Sum, NumericValue};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlock2Sum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyDate2Sum<T, S1T, S2T>,
    pub difficultyepoch: LazyTransform2Sum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDerivedBlock2Sum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSum<S1T>,
        source2: &DerivedComputedBlockSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDate2Sum::from_derived::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
