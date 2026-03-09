use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadableBoxedVec, ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes,
    internal::{ComputedFromHeight, ComputedVecValue, LazyHeightDerived, NumericValue},
};
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromHeight<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyHeightDerived<T, S1T>>,
}

impl<T, S1T> LazyFromHeight<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &ComputedFromHeight<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(name, version, height_source),
            rest: Box::new(LazyHeightDerived::from_computed::<F>(name, version, source)),
        }
    }

    pub(crate) fn from_height_source<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        indexes: &indexes::Vecs,
    ) -> Self
    where
        S1T: NumericValue,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(name, version, height_source.clone()),
            rest: Box::new(LazyHeightDerived::from_height_source::<F>(
                name,
                version,
                height_source,
                indexes,
            )),
        }
    }

    /// Create by unary-transforming a LazyFromHeight source (chaining lazy vecs).
    pub(crate) fn from_lazy<F, S2T>(
        name: &str,
        version: Version,
        source: &LazyFromHeight<S1T, S2T>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S2T: ComputedVecValue + JsonSchema,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(
                name,
                version,
                source.height.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyHeightDerived::from_lazy::<F, S2T>(
                name,
                version,
                &source.rest,
            )),
        }
    }
}
