//! Lazy unary transform for Stats (without percentiles).

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

/// Lazy transform matching Stats structure: average, min, max, sum, cumulative (no percentiles).
#[derive(Clone, Traversable)]
pub struct LazyTransformStats<I, T, S1T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub average: LazyVecFrom1<I, T, I, S1T>,
    pub min: LazyVecFrom1<I, T, I, S1T>,
    pub max: LazyVecFrom1<I, T, I, S1T>,
    pub sum: LazyVecFrom1<I, T, I, S1T>,
    pub cumulative: LazyVecFrom1<I, T, I, S1T>,
}

impl<I, T, S1T> LazyTransformStats<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        average_source: IterableBoxedVec<I, S1T>,
        min_source: IterableBoxedVec<I, S1T>,
        max_source: IterableBoxedVec<I, S1T>,
        sum_source: IterableBoxedVec<I, S1T>,
        cumulative_source: IterableBoxedVec<I, S1T>,
    ) -> Self {
        Self {
            average: LazyVecFrom1::transformed::<F>(
                &format!("{name}_average"),
                version,
                average_source,
            ),
            min: LazyVecFrom1::transformed::<F>(&format!("{name}_min"), version, min_source),
            max: LazyVecFrom1::transformed::<F>(&format!("{name}_max"), version, max_source),
            sum: LazyVecFrom1::transformed::<F>(&format!("{name}_sum"), version, sum_source),
            cumulative: LazyVecFrom1::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cumulative_source,
            ),
        }
    }
}
