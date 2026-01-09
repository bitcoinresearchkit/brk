//! Lazy binary transform for SumCum.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2, VecIndex, VecValue,
};

use crate::internal::{ComputedVecValue, LazyFull, LastVec, SumCum};

#[derive(Clone, Traversable)]
pub struct LazyTransform2SumCum<I, T, S1T = T, S2T = T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub sum: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub cumulative: LazyVecFrom2<I, T, I, S1T, I, S2T>,
}

impl<I, T, S1T, S2T> LazyTransform2SumCum<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &SumCum<I, S1T>,
        source2: &SumCum<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source1.sum.0.boxed_clone(),
                source2.sum.0.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.0.boxed_clone(),
                source2.cumulative.0.boxed_clone(),
            ),
        }
    }

    /// Create from SumCum without adding _sum suffix.
    pub fn from_sum_cum_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &SumCum<I, S1T>,
        source2: &SumCum<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                name,
                version,
                source1.sum.0.boxed_clone(),
                source2.sum.0.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.0.boxed_clone(),
                source2.cumulative.0.boxed_clone(),
            ),
        }
    }

    pub fn from_sources<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: IterableBoxedVec<I, S1T>,
        sum_source2: IterableBoxedVec<I, S2T>,
        cum_source1: IterableBoxedVec<I, S1T>,
        cum_source2: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                sum_source1,
                sum_source2,
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cum_source1,
                cum_source2,
            ),
        }
    }

    /// Create from sources without adding _sum suffix.
    pub fn from_sources_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: IterableBoxedVec<I, S1T>,
        sum_source2: IterableBoxedVec<I, S2T>,
        cum_source1: IterableBoxedVec<I, S1T>,
        cum_source2: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(name, version, sum_source1, sum_source2),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cum_source1,
                cum_source2,
            ),
        }
    }

    pub fn from_lazy_stats_aggregate<F, S1I, S1L, S2I, S2L>(
        name: &str,
        version: Version,
        source1: &LazyFull<I, S1T, S1I, S1L>,
        source2: &LazyFull<I, S2T, S2I, S2L>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1I: VecIndex + 'static,
        S1L: VecValue,
        S2I: VecIndex + 'static,
        S2L: VecValue,
    {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source1.sum.boxed_clone(),
                source2.sum.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.boxed_clone(),
                source2.cumulative.boxed_clone(),
            ),
        }
    }

    /// Create from lazy stats aggregate without adding _sum suffix.
    pub fn from_lazy_stats_aggregate_sum_raw<F, S1I, S1L, S2I, S2L>(
        name: &str,
        version: Version,
        source1: &LazyFull<I, S1T, S1I, S1L>,
        source2: &LazyFull<I, S2T, S2I, S2L>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1I: VecIndex + 'static,
        S1L: VecValue,
        S2I: VecIndex + 'static,
        S2L: VecValue,
    {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                name,
                version,
                source1.sum.boxed_clone(),
                source2.sum.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.boxed_clone(),
                source2.cumulative.boxed_clone(),
            ),
        }
    }

    // --- Methods accepting SumCum + Last sources ---

    pub fn from_sum_cum_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &SumCum<I, S1T>,
        source2: &LastVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                source1.sum.0.boxed_clone(),
                source2.0.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.0.boxed_clone(),
                source2.0.boxed_clone(),
            ),
        }
    }

    /// Create from SumCum + Last without adding _sum suffix.
    pub fn from_sum_cum_last_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &SumCum<I, S1T>,
        source2: &LastVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                name,
                version,
                source1.sum.0.boxed_clone(),
                source2.0.boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.0.boxed_clone(),
                source2.0.boxed_clone(),
            ),
        }
    }

    pub fn from_sources_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: IterableBoxedVec<I, S1T>,
        cum_source1: IterableBoxedVec<I, S1T>,
        last_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(
                &format!("{name}_sum"),
                version,
                sum_source1,
                last_source.clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cum_source1,
                last_source,
            ),
        }
    }

    /// Create from boxed SumCum + Last sources without adding _sum suffix.
    pub fn from_sources_last_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: IterableBoxedVec<I, S1T>,
        cum_source1: IterableBoxedVec<I, S1T>,
        last_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(name, version, sum_source1, last_source.clone()),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cum_source1,
                last_source,
            ),
        }
    }
}
