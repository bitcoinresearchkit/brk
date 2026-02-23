//! Lazy unary transform from height with Last aggregation.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{
    ComputedFromHeightLast,
    ComputedVecValue, LazyBinaryComputedFromHeightLast, LazyBinaryFromHeightLast,
    LazyHeightDerivedLast, NumericValue,
};
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromHeightLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyHeightDerivedLast<T, S1T>>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyFromHeightLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &ComputedFromHeightLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: Box::new(LazyHeightDerivedLast::from_computed::<F>(name, v, source)),
        }
    }

    pub(crate) fn from_lazy_binary_computed<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &LazyBinaryComputedFromHeightLast<S1T, S1aT, S1bT>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S1T: NumericValue,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: Box::new(LazyHeightDerivedLast::from_derived_computed::<F>(name, v, &source.rest)),
        }
    }

    /// Create by unary-transforming a LazyFromHeightLast source (chaining lazy vecs).
    pub(crate) fn from_lazy<F, S2T>(
        name: &str,
        version: Version,
        source: &LazyFromHeightLast<S1T, S2T>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S2T: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, source.height.read_only_boxed_clone()),
            rest: Box::new(LazyHeightDerivedLast::from_lazy::<F, S2T>(name, v, &source.rest)),
        }
    }

    /// Create by unary-transforming a LazyBinaryFromHeightLast source.
    pub(crate) fn from_binary<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source: &LazyBinaryFromHeightLast<S1T, S1aT, S1bT>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, source.height.read_only_boxed_clone()),
            rest: Box::new(LazyHeightDerivedLast::from_binary::<F, _, _>(name, v, &source.rest)),
        }
    }
}
