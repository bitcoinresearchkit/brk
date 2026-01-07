//! Lazy unary transform from height with Last aggregation.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{
    ComputedBlockLast, ComputedVecValue, DerivedComputedBlockLast, NumericValue,
};

use super::super::derived_block::LazyDerivedBlockLast;
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBlockLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: LazyDerivedBlockLast<T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyBlockLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &ComputedBlockLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: LazyDerivedBlockLast::from_computed::<F>(name, v, source),
        }
    }

    pub fn from_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &DerivedComputedBlockLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: LazyDerivedBlockLast::from_derived_computed::<F>(name, v, source),
        }
    }
}
