//! Lazy binary transform from two SumCum sources, producing Last (cumulative) ratios only.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedHeightDateLast, ComputedVecValue,
    DerivedDateLast, LazyBinaryDateLast, LazyTransform2Last, NumericValue,
};

use super::super::derived_block::LazyDerivedBlock2Last;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryBlockLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: LazyDerivedBlock2Last<T, S1T, S2T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyBinaryBlockLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockSumCum<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height_cumulative.0.boxed_clone(),
                source2.height_cumulative.0.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last::from_computed_sum_cum::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockLast<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last::from_computed_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_height_date_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDateLast<S1T>,
        source2: &ComputedHeightDateLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last::from_computed_height_date_last::<F>(
                name, v, source1, source2,
            ),
        }
    }

    /// Create from a ComputedHeightDateLast and a LazyBinaryBlockLast.
    pub fn from_computed_height_date_and_binary_block<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDateLast<S1T>,
        source2: &LazyBinaryBlockLast<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: PartialOrd,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last {
                dates: LazyBinaryDateLast::from_computed_and_binary_last::<F, _, _>(
                    name,
                    v,
                    &source1.rest,
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyTransform2Last::from_vecs::<F>(
                    name,
                    v,
                    source1.difficultyepoch.0.boxed_clone(),
                    source2.rest.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from a ComputedHeightDateLast and a ComputedBlockLast.
    pub fn from_computed_height_date_and_block_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDateLast<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last::from_computed_height_date_and_block_last::<F>(
                name, v, source1, source2,
            ),
        }
    }

    /// Create from a LazyBinaryBlockLast and a ComputedBlockLast.
    pub fn from_binary_block_and_computed_block_last<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryBlockLast<S1T, S1aT, S1bT>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last {
                dates: LazyBinaryDateLast::from_binary_and_block_last::<F, _, _>(
                    name,
                    v,
                    &source1.rest.dates,
                    source2,
                ),
                difficultyepoch: LazyTransform2Last::from_vecs::<F>(
                    name,
                    v,
                    source1.rest.difficultyepoch.boxed_clone(),
                    source2.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from two LazyBinaryBlockLast sources.
    pub fn from_both_binary_block<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryBlockLast<S1T, S1aT, S1bT>,
        source2: &LazyBinaryBlockLast<S2T, S2aT, S2bT>,
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
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyDerivedBlock2Last {
                dates: LazyBinaryDateLast::from_both_binary_last::<F, _, _, _, _>(
                    name,
                    v,
                    &source1.rest.dates,
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyTransform2Last::from_vecs::<F>(
                    name,
                    v,
                    source1.rest.difficultyepoch.boxed_clone(),
                    source2.rest.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from separate height, difficultyepoch, and date sources.
    ///
    /// Use when sources are split across different types (e.g., ValueBlockDateLast + ComputedBlockLast).
    #[allow(clippy::too_many_arguments)]
    pub fn from_height_difficultyepoch_dates<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        difficultyepoch_source1: IterableBoxedVec<DifficultyEpoch, S1T>,
        difficultyepoch_source2: IterableBoxedVec<DifficultyEpoch, S2T>,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        dates_source1: &DerivedDateLast<S1T>,
        dateindex_source2: IterableBoxedVec<DateIndex, S2T>,
        dates_source2: &DerivedDateLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: LazyDerivedBlock2Last {
                dates: LazyBinaryDateLast::from_both_derived_last::<F>(
                    name,
                    v,
                    dateindex_source1,
                    dates_source1,
                    dateindex_source2,
                    dates_source2,
                ),
                difficultyepoch: LazyTransform2Last::from_vecs::<F>(
                    name,
                    v,
                    difficultyepoch_source1,
                    difficultyepoch_source2,
                ),
            },
        }
    }
}
