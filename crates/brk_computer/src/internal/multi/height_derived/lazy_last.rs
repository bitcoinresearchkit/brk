//! Lazy aggregated Last for block-level sources.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedFromHeightLast, ComputedHeightDerivedLast, ComputedFromHeightAndDateLast, ComputedVecValue,
    LazyFromDateLast, LazyTransformLast, NumericValue,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyHeightDerivedLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyFromDateLast<T, S1T>,
    pub difficultyepoch: LazyTransformLast<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyFromDateLast::from_derived::<F>(
                name,
                v,
                source.dateindex.boxed_clone(),
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
        source: &ComputedHeightDerivedLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyFromDateLast::from_derived::<F>(
                name,
                v,
                source.dateindex.boxed_clone(),
                &source.dates,
            ),
            difficultyepoch: LazyTransformLast::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_height_date<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightAndDateLast<S1T>,
    ) -> Self
    where
        S1T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            dates: LazyFromDateLast::from_derived::<F>(
                name,
                v,
                source.dateindex.boxed_clone(),
                &source.rest.rest,
            ),
            difficultyepoch: LazyTransformLast::from_boxed::<F>(
                name,
                v,
                source.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
