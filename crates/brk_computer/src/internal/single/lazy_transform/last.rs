//! Lazy unary transform for Last-only - transforms last at one index level.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadableBoxedVec, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
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
    pub(crate) fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        last_source: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(name, version, last_source))
    }
}
