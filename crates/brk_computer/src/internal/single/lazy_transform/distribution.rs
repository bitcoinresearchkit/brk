//! Lazy unary transform for Distribution metrics.
//! Has average, min, max, and percentiles - but no sum/cumulative.
//! Use for ratio/percentage metrics where aggregation doesn't make sense.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

use super::LazyPercentiles;

/// Distribution stats: average, min, max, percentiles.
/// Excludes sum and cumulative (meaningless for ratios/percentages).
#[derive(Clone, Traversable)]
pub struct LazyTransformDistribution<I, T, S1T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub average: LazyVecFrom1<I, T, I, S1T>,
    pub min: LazyVecFrom1<I, T, I, S1T>,
    pub max: LazyVecFrom1<I, T, I, S1T>,
    #[traversable(flatten)]
    pub percentiles: LazyPercentiles<I, T, S1T>,
}

impl<I, T, S1T> LazyTransformDistribution<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        average: ReadableBoxedVec<I, S1T>,
        min: ReadableBoxedVec<I, S1T>,
        max: ReadableBoxedVec<I, S1T>,
        pct10: ReadableBoxedVec<I, S1T>,
        pct25: ReadableBoxedVec<I, S1T>,
        median: ReadableBoxedVec<I, S1T>,
        pct75: ReadableBoxedVec<I, S1T>,
        pct90: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self {
            average: LazyVecFrom1::transformed::<F>(&format!("{name}_average"), version, average),
            min: LazyVecFrom1::transformed::<F>(&format!("{name}_min"), version, min),
            max: LazyVecFrom1::transformed::<F>(&format!("{name}_max"), version, max),
            percentiles: LazyPercentiles::from_boxed::<F>(name, version, pct10, pct25, median, pct75, pct90),
        }
    }
}
