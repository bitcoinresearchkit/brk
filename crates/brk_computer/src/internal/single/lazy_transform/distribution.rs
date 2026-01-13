//! Lazy unary transform for Distribution metrics.
//! Has average, min, max, and percentiles - but no sum/cumulative.
//! Use for ratio/percentage metrics where aggregation doesn't make sense.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::{ComputedVecValue, Full};

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
    pub fn from_stats_aggregate<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &Full<I, S1T>,
    ) -> Self {
        Self {
            average: LazyVecFrom1::transformed::<F>(
                &format!("{name}_average"),
                version,
                source.boxed_average(),
            ),
            min: LazyVecFrom1::transformed::<F>(
                &format!("{name}_min"),
                version,
                source.boxed_min(),
            ),
            max: LazyVecFrom1::transformed::<F>(
                &format!("{name}_max"),
                version,
                source.boxed_max(),
            ),
            percentiles: LazyPercentiles::from_percentiles::<F>(
                name,
                version,
                &source.distribution.percentiles,
            ),
        }
    }
}
