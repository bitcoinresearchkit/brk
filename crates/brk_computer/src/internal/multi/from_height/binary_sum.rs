//! Lazy binary transform from two Sum-only sources with height level.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedFromHeightSum, ComputedFromHeightSumCum, ComputedHeightDerivedSum, ComputedVecValue,
    LazyBinaryHeightDerivedSum, LazyFromHeightLast, NumericValue,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryFromHeightSum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: LazyBinaryHeightDerivedSum<T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryFromHeightSum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedHeightDerivedSum<S1T>,
        source2: &ComputedHeightDerivedSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: LazyBinaryHeightDerivedSum::from_derived::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSum<S1T>,
        source2: &ComputedFromHeightSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.height.boxed_clone(),
                source2.height.boxed_clone(),
            ),
            rest: LazyBinaryHeightDerivedSum::from_derived::<F>(name, v, &source1.rest, &source2.rest),
        }
    }

    /// Create from two LazyBinaryFromHeightSum sources.
    pub fn from_binary<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryFromHeightSum<S1T, S1aT, S1bT>,
        source2: &LazyBinaryFromHeightSum<S2T, S2aT, S2bT>,
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
            rest: LazyBinaryHeightDerivedSum::from_binary::<F, _, _, _, _>(
                name,
                v,
                &source1.rest,
                &source2.rest,
            ),
        }
    }

    /// Create from a SumCum source (using only sum) and a LazyLast source.
    /// Produces sum-only output (no cumulative).
    pub fn from_sumcum_lazy_last<F, S2ST>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &LazyFromHeightLast<S2T, S2ST>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2ST: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: LazyBinaryHeightDerivedSum::from_sumcum_lazy_last::<F, S2ST>(
                name,
                v,
                source1,
                source2,
            ),
        }
    }
}
