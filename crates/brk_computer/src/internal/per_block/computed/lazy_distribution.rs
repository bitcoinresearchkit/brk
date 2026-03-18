use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadableCloneableVec, UnaryTransform, VecIndex};

use crate::internal::{ComputedVecValue, PerBlockDistribution, DistributionStats};

/// Lazy analog of `Distribution<T>`: 8 `LazyVecFrom1` fields,
/// each derived by transforming the corresponding field of a source `PerBlockDistribution<S1T>`.
#[derive(Clone, Traversable)]
pub struct LazyDistribution<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    pub average: LazyVecFrom1<I, T, I, S1T>,
    pub min: LazyVecFrom1<I, T, I, S1T>,
    pub max: LazyVecFrom1<I, T, I, S1T>,
    pub pct10: LazyVecFrom1<I, T, I, S1T>,
    pub pct25: LazyVecFrom1<I, T, I, S1T>,
    pub median: LazyVecFrom1<I, T, I, S1T>,
    pub pct75: LazyVecFrom1<I, T, I, S1T>,
    pub pct90: LazyVecFrom1<I, T, I, S1T>,
}

impl<T, S1T> LazyDistribution<Height, T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub(crate) fn from_distribution<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &PerBlockDistribution<S1T>,
    ) -> Self {
        let s = DistributionStats::<()>::SUFFIXES;
        Self {
            average: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[0]),
                version,
                source.average.height.read_only_boxed_clone(),
            ),
            min: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[1]),
                version,
                source.min.height.read_only_boxed_clone(),
            ),
            max: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[2]),
                version,
                source.max.height.read_only_boxed_clone(),
            ),
            pct10: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[3]),
                version,
                source.pct10.height.read_only_boxed_clone(),
            ),
            pct25: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[4]),
                version,
                source.pct25.height.read_only_boxed_clone(),
            ),
            median: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[5]),
                version,
                source.median.height.read_only_boxed_clone(),
            ),
            pct75: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[6]),
                version,
                source.pct75.height.read_only_boxed_clone(),
            ),
            pct90: LazyVecFrom1::transformed::<F>(
                &format!("{name}_{}", s[7]),
                version,
                source.pct90.height.read_only_boxed_clone(),
            ),
        }
    }
}
