//! Lazy binary transform from two SumCum sources.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedVecValue, DerivedComputedBlockLast,
    DerivedComputedBlockSumCum, NumericValue,
};

use super::super::derived_block::LazyDerivedBlock2SumCum;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct BinaryBlockSumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(wrap = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[traversable(rename = "cumulative")]
    pub height_cumulative: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: LazyDerivedBlock2SumCum<T, S1T, S2T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> BinaryBlockSumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockSumCum<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                source2.height_cumulative.0.boxed_clone(),
            ),
            rest: LazyDerivedBlock2SumCum::from_computed_sum_raw::<F>(
                name,
                v,
                &source1.dateindex,
                &source1.rest,
                &source1.difficultyepoch,
                &source2.dateindex,
                &source2.rest,
                &source2.difficultyepoch,
            ),
        }
    }

    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockSumCum<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                source2.height_cumulative.0.boxed_clone(),
            ),
            rest: LazyDerivedBlock2SumCum::from_computed_sum_raw::<F>(
                name,
                v,
                &source1.dateindex,
                source1,
                &source1.difficultyepoch,
                &source2.dateindex,
                source2,
                &source2.difficultyepoch,
            ),
        }
    }

    // --- Methods accepting SumCum + Last sources ---

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2SumCum::from_computed_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_derived_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2SumCum::from_derived_computed_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1.clone(), height_source2.clone()),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                height_source2,
            ),
            rest: LazyDerivedBlock2SumCum::from_derived_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1.clone(), height_source2.clone()),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.0.boxed_clone(),
                height_source2,
            ),
            rest: LazyDerivedBlock2SumCum::from_computed_derived_last::<F>(name, v, source1, source2),
        }
    }
}
