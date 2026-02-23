//! Lazy unary transform for SumCum.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
pub struct LazyTransformSumCum<I, T, S1T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub sum: LazyVecFrom1<I, T, I, S1T>,
    pub cumulative: LazyVecFrom1<I, T, I, S1T>,
}

impl<I, T, S1T> LazyTransformSumCum<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_boxed_sum_raw<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        sum_source: ReadableBoxedVec<I, S1T>,
        cumulative_source: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom1::transformed::<F>(name, version, sum_source),
            cumulative: LazyVecFrom1::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cumulative_source,
            ),
        }
    }
}
