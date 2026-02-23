//! Lazy binary transform from two SumCum sources, producing Last (cumulative) ratios only.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedHeightDerivedLast, ComputedVecValue,
    LazyBinaryComputedFromHeightLast, LazyBinaryComputedFromHeightSum,
    LazyBinaryHeightDerivedLast, LazyBinaryTransformLast,
    LazyFromHeightLast, NumericValue,
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
    pub rest: Box<LazyBinaryHeightDerivedLast<T, S1T, S2T>>,
}

const VERSION: Version = Version::ZERO;

/// Helper macro: given two deref-able sources whose `.$p` fields implement
/// `ReadableCloneableVec`, build all 17 period fields of a `LazyBinaryHeightDerivedLast`.
macro_rules! build_rest {
    ($name:expr, $v:expr, $source1:expr, $source2:expr) => {{
        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    $name,
                    $v,
                    $source1.$p.read_only_boxed_clone(),
                    $source2.$p.read_only_boxed_clone(),
                )
            };
        }
        Box::new(LazyBinaryHeightDerivedLast {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        })
    }};
}

impl<T, S1T, S2T> LazyBinaryFromHeightLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
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
                source1.height_cumulative.read_only_boxed_clone(),
                source2.height_cumulative.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedLast::from_computed_sum_cum::<F>(name, v, source1, source2)),
        }
    }

    pub(crate) fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
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
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedLast::from_computed_last::<F>(name, v, source1, source2)),
        }
    }

    pub(crate) fn from_block_last_and_lazy_block_last<F, S2SourceT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &LazyFromHeightLast<S2T, S2SourceT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: NumericValue,
        S2SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedLast::from_block_last_and_lazy_block_last::<F, _>(
                name, v, source1, source2,
            )),
        }
    }

    pub(crate) fn from_lazy_block_last_and_block_last<F, S1SourceT>(
        name: &str,
        version: Version,
        source1: &LazyFromHeightLast<S1T, S1SourceT>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2T: NumericValue,
        S1SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedLast::from_lazy_block_last_and_block_last::<F, _>(
                name, v, source1, source2,
            )),
        }
    }

    /// Create from a ComputedFromHeightLast and a LazyBinaryFromHeightLast.
    pub(crate) fn from_block_last_and_binary_block<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &LazyBinaryFromHeightLast<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: NumericValue,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: build_rest!(name, v, source1, source2),
        }
    }

    /// Create from two LazyBinaryFromHeightLast sources.
    pub(crate) fn from_both_binary_block<F, S1aT, S1bT, S2aT, S2bT>(
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
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: build_rest!(name, v, source1, source2),
        }
    }

    /// Create from separate height sources and two `ComputedHeightDerivedLast` structs.
    pub(crate) fn from_height_and_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: ReadableBoxedVec<Height, S1T>,
        height_source2: ReadableBoxedVec<Height, S2T>,
        derived1: &ComputedHeightDerivedLast<S1T>,
        derived2: &ComputedHeightDerivedLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: build_rest!(name, v, derived1, derived2),
        }
    }

    /// Create from a ComputedFromHeightLast and a LazyBinaryComputedFromHeightLast.
    pub(crate) fn from_block_last_and_lazy_binary_computed_block_last<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &LazyBinaryComputedFromHeightLast<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: NumericValue,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: build_rest!(name, v, source1, source2),
        }
    }

    /// Create from a LazyBinaryComputedFromHeightLast and a LazyBinaryComputedFromHeightSum.
    pub(crate) fn from_lazy_binary_block_last_and_lazy_binary_sum<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryComputedFromHeightLast<S1T, S1aT, S1bT>,
        source2: &LazyBinaryComputedFromHeightSum<S2T, S2aT, S2bT>,
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
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: build_rest!(name, v, source1, source2),
        }
    }

    /// Create from a LazyBinaryFromHeightLast and a LazyBinaryComputedFromHeightLast.
    pub(crate) fn from_binary_block_and_lazy_binary_block_last<F, S1aT, S1bT, S2aT, S2bT>(
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
                source1.height.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: build_rest!(name, v, source1, source2),
        }
    }
}
