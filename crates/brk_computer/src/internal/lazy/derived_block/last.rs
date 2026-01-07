//! Lazy aggregated Last for block-level sources.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedBlockLast, ComputedVecValue, DerivedComputedBlockLast, LazyDateLast, NumericValue,
};

use super::super::transform::LazyTransformLast;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlockLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyDateLast<T, S1T>,
    pub difficultyepoch: LazyTransformLast<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyDerivedBlockLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedBlockLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDateLast::from_derived::<F>(
                name,
                v,
                source.dateindex.0.boxed_clone(),
                &source.rest,
            ),
            difficultyepoch: LazyTransformLast::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &DerivedComputedBlockLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDateLast::from_derived::<F>(
                name,
                v,
                source.dateindex.0.boxed_clone(),
                &source.dates,
            ),
            difficultyepoch: LazyTransformLast::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
