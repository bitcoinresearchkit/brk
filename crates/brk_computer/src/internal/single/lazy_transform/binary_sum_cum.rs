//! Lazy binary transform for SumCum.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, LazyVecFrom2, ReadableBoxedVec, ReadableCloneableVec, VecIndex, VecValue,
};

use crate::internal::{ComputedVecValue, LazyFull};

#[derive(Clone, Traversable)]
pub struct LazyBinaryTransformSumCum<I, T, S1T = T, S2T = T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub sum: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub cumulative: LazyVecFrom2<I, T, I, S1T, I, S2T>,
}

impl<I, T, S1T, S2T> LazyBinaryTransformSumCum<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create from sources without adding _sum suffix.
    pub(crate) fn from_sources_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: ReadableBoxedVec<I, S1T>,
        sum_source2: ReadableBoxedVec<I, S2T>,
        cum_source1: ReadableBoxedVec<I, S1T>,
        cum_source2: ReadableBoxedVec<I, S2T>,
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

    pub(crate) fn from_lazy_stats_aggregate<F, S1I, S1L, S2I, S2L>(
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
                source1.sum.read_only_boxed_clone(),
                source2.sum.read_only_boxed_clone(),
            ),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                source1.cumulative.read_only_boxed_clone(),
                source2.cumulative.read_only_boxed_clone(),
            ),
        }
    }

    /// Create from boxed SumCum + Last sources without adding _sum suffix.
    pub(crate) fn from_sources_last_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source1: ReadableBoxedVec<I, S1T>,
        cum_source1: ReadableBoxedVec<I, S1T>,
        last_source: ReadableBoxedVec<I, S2T>,
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
