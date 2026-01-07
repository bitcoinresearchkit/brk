//! Lazy unary transform for Last-only - transforms last at one index level.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    IterableBoxedVec, IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecIndex, VecValue,
};

use crate::internal::{ComputedVecValue, LastVec, LazyLast};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(wrap = "last")]
pub struct LazyTransformLast<I, T, S1T = T>(pub LazyVecFrom1<I, T, I, S1T>)
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue;

impl<I, T, S1T> LazyTransformLast<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_last_vec<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &LastVec<I, S1T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(
            name,
            version,
            source.0.boxed_clone(),
        ))
    }

    pub fn from_lazy_last<F: UnaryTransform<S1T, T>, S1I: VecIndex + 'static, S1S2T: VecValue>(
        name: &str,
        version: Version,
        source: &LazyLast<I, S1T, S1I, S1S2T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(
            name,
            version,
            source.boxed_clone(),
        ))
    }

    pub fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        last_source: IterableBoxedVec<I, S1T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(name, version, last_source))
    }
}
