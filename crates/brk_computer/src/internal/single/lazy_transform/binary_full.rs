//! Lazy binary transform for Full (without cumulative).
//!
//! Used for USD conversion where `usd = sats * price[height]`.
//! Cumulative cannot be lazy because `cum_usd ≠ cum_sats * price` -
//! it must be computed by summing historical `sum * price` values.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2, VecIndex};

use crate::internal::{ComputedVecValue, Full};

use super::LazyBinaryPercentiles;

/// Lazy binary transform for Full stats (excluding cumulative).
///
/// For USD conversion: each stat is computed as `sats_stat * price`.
/// Cumulative is excluded because it requires summing historical values.
#[derive(Clone, Traversable)]
pub struct LazyBinaryTransformFull<I, T, S1T = T, S2T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub average: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub min: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub max: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    #[traversable(flatten)]
    pub percentiles: LazyBinaryPercentiles<I, T, S1T, S2T>,
    pub sum: LazyVecFrom2<I, T, I, S1T, I, S2T>,
}

impl<I, T, S1T, S2T> LazyBinaryTransformFull<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create from Full source and a second source (e.g., price).
    ///
    /// The transform F is applied as `F(source1_stat, source2)` for each stat.
    pub fn from_full_and_source<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &Full<I, S1T>,
        source2: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            average: LazyVecFrom2::transformed::<F>(
                &format!("{name}_average"),
                version,
                source1.boxed_average(),
                source2.clone(),
            ),
            min: LazyVecFrom2::transformed::<F>(
                &format!("{name}_min"),
                version,
                source1.boxed_min(),
                source2.clone(),
            ),
            max: LazyVecFrom2::transformed::<F>(
                &format!("{name}_max"),
                version,
                source1.boxed_max(),
                source2.clone(),
            ),
            percentiles: LazyBinaryPercentiles::from_percentiles::<F>(
                name,
                version,
                &source1.distribution.percentiles,
                source2.clone(),
            ),
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source1.boxed_sum(),
                source2,
            ),
        }
    }

    pub fn boxed_average(&self) -> IterableBoxedVec<I, T> {
        self.average.boxed_clone()
    }

    pub fn boxed_min(&self) -> IterableBoxedVec<I, T> {
        self.min.boxed_clone()
    }

    pub fn boxed_max(&self) -> IterableBoxedVec<I, T> {
        self.max.boxed_clone()
    }

    pub fn boxed_sum(&self) -> IterableBoxedVec<I, T> {
        self.sum.boxed_clone()
    }
}
