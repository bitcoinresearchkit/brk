//! Lazy unary transform from height with Sum aggregation.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{ComputedBlockSum, ComputedVecValue, DerivedComputedBlockSum, NumericValue};

use super::super::derived_block::LazyDerivedBlockSum;
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBlockSum<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: LazyDerivedBlockSum<T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyBlockSum<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &ComputedBlockSum<S1T>,
    ) -> Self {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: LazyDerivedBlockSum::from_computed::<F>(
                name,
                v,
                &source.dateindex,
                &source.rest,
                &source.difficultyepoch,
            ),
        }
    }

    pub fn from_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &DerivedComputedBlockSum<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            rest: LazyDerivedBlockSum::from_derived_computed::<F>(name, v, source),
        }
    }
}
