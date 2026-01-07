//! Lazy unary transform for Full.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::{ComputedVecValue, Full};

#[derive(Clone, Traversable)]
pub struct LazyTransformFull<I, T, S1T = T>
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

impl<I, T, S1T> LazyTransformFull<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_stats_aggregate<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &Full<I, S1T>,
    ) -> Self {
        Self {
            average: LazyVecFrom1::transformed::<F>(
                &format!("{name}_average"),
                version,
                source.distribution.average.0.boxed_clone(),
            ),
            min: LazyVecFrom1::transformed::<F>(
                &format!("{name}_min"),
                version,
                source.distribution.minmax.min.0.boxed_clone(),
            ),
            max: LazyVecFrom1::transformed::<F>(
                &format!("{name}_max"),
                version,
                source.distribution.minmax.max.0.boxed_clone(),
            ),
            sum: LazyVecFrom1::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source.sum_cum.sum.0.boxed_clone(),
            ),
            cumulative: LazyVecFrom1::transformed::<F>(
                &format!("{name}_cum"),
                version,
                source.sum_cum.cumulative.0.boxed_clone(),
            ),
        }
    }

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
                &format!("{name}_cum"),
                version,
                cumulative_source,
            ),
        }
    }
}
