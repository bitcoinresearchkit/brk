//! Lazy unary transform for Sum-only.

use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(wrap = "sum")]
pub struct LazyTransformSum<I, T, S1T = T>(pub LazyVecFrom1<I, T, I, S1T>)
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue;

impl<I, T, S1T> LazyTransformSum<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        sum_source: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(name, version, sum_source))
    }
}
