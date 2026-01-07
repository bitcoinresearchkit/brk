//! Lazy aggregated for SumCum + Last binary transform.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedVecValue, DerivedComputedBlockLast,
    DerivedComputedBlockSumCum, LazyDate2SumCumLast, NumericValue,
};

use super::super::transform::LazyTransform2SumCumLast;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlock2SumCumLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyDate2SumCumLast<T, S1T, S2T>,
    pub difficultyepoch: LazyTransform2SumCumLast<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDerivedBlock2SumCumLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCumLast::from_computed::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed_full<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCumLast::from_derived_computed_full::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_derived_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCumLast::from_computed_derived_computed::<F>(
                name, v, source1, source2,
            ),
            difficultyepoch: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCumLast::from_derived_computed::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
