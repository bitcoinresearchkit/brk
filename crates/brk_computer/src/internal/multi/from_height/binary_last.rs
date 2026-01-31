//! Lazy binary transform from two SumCum sources, producing Last (cumulative) ratios only.

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedFromHeightAndDateLast, ComputedVecValue,
    LazyBinaryComputedFromHeightLast, LazyBinaryFromDateLast, LazyBinaryHeightDerivedLast,
    LazyBinaryTransformLast, LazyDateDerivedLast, LazyFromHeightLast, NumericValue,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryFromHeightLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: LazyBinaryHeightDerivedLast<T, S1T, S2T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyBinaryFromHeightLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &ComputedFromHeightSumCum<S2T>,
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
                source1.height_cumulative.boxed_clone(),
                source2.height_cumulative.boxed_clone(),
            ),
            rest: LazyBinaryHeightDerivedLast::from_computed_sum_cum::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
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
            rest: LazyBinaryHeightDerivedLast::from_computed_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_height_date_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &ComputedFromHeightAndDateLast<S2T>,
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
            rest: LazyBinaryHeightDerivedLast::from_computed_height_date_last::<F>(
                name, v, source1, source2,
            ),
        }
    }

    /// Create from a ComputedFromHeightAndDateLast and a LazyBinaryFromHeightLast.
    pub fn from_computed_height_date_and_binary_block<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &LazyBinaryFromHeightLast<S2T, S2aT, S2bT>,
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
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_computed_and_binary_last::<F, _, _>(
                    name,
                    v,
                    &source1.rest,
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.difficultyepoch.boxed_clone(),
                    source2.rest.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from a ComputedFromHeightAndDateLast and a ComputedFromHeightLast.
    pub fn from_computed_height_date_and_block_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
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
            rest: LazyBinaryHeightDerivedLast::from_computed_height_date_and_block_last::<F>(
                name, v, source1, source2,
            ),
        }
    }

    /// Create from a LazyBinaryFromHeightLast and a ComputedFromHeightLast.
    pub fn from_binary_block_and_computed_block_last<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryFromHeightLast<S1T, S1aT, S1bT>,
        source2: &ComputedFromHeightLast<S2T>,
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
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_binary_and_block_last::<F, _, _>(
                    name,
                    v,
                    &source1.rest.dates,
                    source2,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.rest.difficultyepoch.boxed_clone(),
                    source2.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from two LazyBinaryFromHeightLast sources.
    pub fn from_both_binary_block<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryFromHeightLast<S1T, S1aT, S1bT>,
        source2: &LazyBinaryFromHeightLast<S2T, S2aT, S2bT>,
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
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_both_binary_last::<F, _, _, _, _>(
                    name,
                    v,
                    &source1.rest.dates,
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
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
    /// Use when sources are split across different types (e.g., ValueFromHeightAndDateLast + ComputedFromHeightLast).
    #[allow(clippy::too_many_arguments)]
    pub fn from_height_difficultyepoch_dates<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        difficultyepoch_source1: IterableBoxedVec<DifficultyEpoch, S1T>,
        difficultyepoch_source2: IterableBoxedVec<DifficultyEpoch, S2T>,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        dates_source1: &LazyDateDerivedLast<S1T>,
        dateindex_source2: IterableBoxedVec<DateIndex, S2T>,
        dates_source2: &LazyDateDerivedLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_both_derived_last::<F>(
                    name,
                    v,
                    dateindex_source1,
                    dates_source1,
                    dateindex_source2,
                    dates_source2,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    difficultyepoch_source1,
                    difficultyepoch_source2,
                ),
            },
        }
    }

    /// Create from a ComputedFromHeightAndDateLast and a LazyBinaryComputedFromHeightLast.
    pub fn from_computed_height_date_and_lazy_binary_block_last<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &LazyBinaryComputedFromHeightLast<S2T, S2aT, S2bT>,
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
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_both_derived_last::<F>(
                    name,
                    v,
                    source1.rest.dateindex.boxed_clone(),
                    &source1.rest.rest,
                    source2.rest.dateindex.boxed_clone(),
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.difficultyepoch.boxed_clone(),
                    source2.rest.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from a LazyBinaryFromHeightLast and a LazyBinaryComputedFromHeightLast.
    pub fn from_binary_block_and_lazy_binary_block_last<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryFromHeightLast<S1T, S1aT, S1bT>,
        source2: &LazyBinaryComputedFromHeightLast<S2T, S2aT, S2bT>,
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
            rest: LazyBinaryHeightDerivedLast {
                dates: LazyBinaryFromDateLast::from_binary_and_derived_last::<F, _, _>(
                    name,
                    v,
                    &source1.rest.dates,
                    source2.rest.dateindex.boxed_clone(),
                    &source2.rest.dates,
                ),
                difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.rest.difficultyepoch.boxed_clone(),
                    source2.rest.difficultyepoch.boxed_clone(),
                ),
            },
        }
    }

    /// Create from a ComputedFromHeightAndDateLast and a LazyFromHeightLast.
    pub fn from_computed_height_date_and_lazy_block_last<F, S2SourceT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &LazyFromHeightLast<S2T, S2SourceT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: PartialOrd,
        S2SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyBinaryHeightDerivedLast::from_computed_height_date_and_lazy_block_last::<F, _>(
                name, v, source1, source2,
            ),
        }
    }
}
