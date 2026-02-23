//! Lazy unary transform from height with SumCum aggregation.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{
    ComputedFromHeightSumCum, ComputedHeightDerivedSumCum, ComputedVecValue,
    LazyHeightDerivedSumCum, NumericValue,
};
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromHeightSumCum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyHeightDerivedSumCum<T, S1T>>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyFromHeightSumCum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &ComputedFromHeightSumCum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: Box::new(LazyHeightDerivedSumCum::from_derived_computed::<F>(name, v, &source.rest)),
        }
    }

    pub(crate) fn from_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &ComputedHeightDerivedSumCum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: Box::new(LazyHeightDerivedSumCum::from_derived_computed::<F>(name, v, source)),
        }
    }
}
