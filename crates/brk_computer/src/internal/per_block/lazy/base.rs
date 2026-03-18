use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes,
    internal::{ComputedVecValue, DerivedResolutions, NumericValue, PerBlock},
};
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyPerBlock<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<DerivedResolutions<T, S1T>>,
}

impl<T, S1T> LazyPerBlock<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        source: &PerBlock<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(name, version, height_source),
            resolutions: Box::new(DerivedResolutions::from_computed::<F>(
                name, version, source,
            )),
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
            resolutions: Box::new(DerivedResolutions::from_height_source::<F>(
                name,
                version,
                height_source,
                indexes,
            )),
        }
    }

    /// Create by unary-transforming a LazyPerBlock source (chaining lazy vecs).
    pub(crate) fn from_lazy<F, S2T>(
        name: &str,
        version: Version,
        source: &LazyPerBlock<S1T, S2T>,
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
            resolutions: Box::new(DerivedResolutions::from_lazy::<F, S2T>(
                name,
                version,
                &source.resolutions,
            )),
        }
    }
}

impl<T, S1T> ReadOnlyClone for LazyPerBlock<T, S1T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    type ReadOnly = Self;
    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}
