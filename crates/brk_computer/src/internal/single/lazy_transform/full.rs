//! Lazy unary transform for Full.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

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
    pub(crate) fn from_stats_aggregate<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &Full<I, S1T>,
    ) -> Self {
        Self::from_boxed::<F>(
            name,
            version,
            source.boxed_average(),
            source.boxed_min(),
            source.boxed_max(),
            source.distribution.percentiles.boxed_pct10(),
            source.distribution.percentiles.boxed_pct25(),
            source.distribution.percentiles.boxed_median(),
            source.distribution.percentiles.boxed_pct75(),
            source.distribution.percentiles.boxed_pct90(),
            source.boxed_sum(),
            source.boxed_cumulative(),
        )
    }

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
        sum: ReadableBoxedVec<I, S1T>,
        cumulative: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self {
            average: LazyVecFrom1::transformed::<F>(&format!("{name}_average"), version, average),
            min: LazyVecFrom1::transformed::<F>(&format!("{name}_min"), version, min),
            max: LazyVecFrom1::transformed::<F>(&format!("{name}_max"), version, max),
            percentiles: LazyPercentiles::from_boxed::<F>(name, version, pct10, pct25, median, pct75, pct90),
            sum: LazyVecFrom1::transformed::<F>(&format!("{name}_sum"), version, sum),
            cumulative: LazyVecFrom1::transformed::<F>(&format!("{name}_cumulative"), version, cumulative),
        }
    }
}
