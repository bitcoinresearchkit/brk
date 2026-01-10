//! Lazy unary transform for Full.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::{ComputedVecValue, Full};

use super::LazyPercentiles;

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
    #[traversable(flatten)]
    pub percentiles: LazyPercentiles<I, T, S1T>,
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
            sum: LazyVecFrom1::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source.boxed_sum(),
            ),
            cumulative: LazyVecFrom1::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source.boxed_cumulative(),
            ),
        }
    }

}
