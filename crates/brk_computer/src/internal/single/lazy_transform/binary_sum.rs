//! Lazy binary transform for Sum-only aggregation at a single index level.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, LazyVecFrom2, VecIndex};

use crate::internal::{ComputedVecValue, SumVec};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(wrap = "sum")]
pub struct LazyBinaryTransformSum<I, T, S1T, S2T>(pub LazyVecFrom2<I, T, I, S1T, I, S2T>)
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue;

impl<I, T, S1T, S2T> LazyBinaryTransformSum<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_sum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &SumVec<I, S1T>,
        source2: &SumVec<I, S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self(LazyVecFrom2::transformed::<F>(
            name,
            v,
            source1.boxed_clone(),
            source2.boxed_clone(),
        ))
    }

    pub fn from_boxed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: IterableBoxedVec<I, S1T>,
        source2: IterableBoxedVec<I, S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self(LazyVecFrom2::transformed::<F>(name, v, source1, source2))
    }
}

