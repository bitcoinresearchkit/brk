//! Lazy binary transform from two Sum-only sources with height level.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedFromHeightSum, ComputedHeightDerivedSum, ComputedVecValue, LazyBinaryHeightDerivedSum,
    NumericValue,
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
}
