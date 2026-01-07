//! Lazy binary transform for SumCum + Last → SumCum result.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2, VecIndex};

use crate::internal::{ComputedVecValue, LastVec, SumCum};

#[derive(Clone, Traversable)]
pub struct LazyTransform2SumCumLast<I, T, S1T = T, S2T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub sum: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub cumulative: LazyVecFrom2<I, T, I, S1T, I, S2T>,
}

impl<I, T, S1T, S2T> LazyTransform2SumCumLast<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_sources<F: BinaryTransform<S1T, S2T, T>>(
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

    pub fn from_boxed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        sum_source: IterableBoxedVec<I, S1T>,
        cum_source: IterableBoxedVec<I, S1T>,
        last_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazyVecFrom2::transformed::<F>(name, version, sum_source, last_source.clone()),
            cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                version,
                cum_source,
                last_source,
            ),
        }
    }
}
