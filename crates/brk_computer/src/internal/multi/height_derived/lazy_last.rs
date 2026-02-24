//! Lazy aggregated Last for block-level sources.
//!
//! Newtype on `Indexes` with `LazyTransformLast` per field.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes_from,
    internal::{
        ComputedFromHeightLast, ComputedHeightDerivedLast, ComputedVecValue, Indexes,
        LazyBinaryHeightDerivedLast, LazyTransformLast, NumericValue,
    },
};

pub type LazyHeightDerivedLastInner<T, S1T> = Indexes<
    LazyTransformLast<Minute1, T, S1T>,
    LazyTransformLast<Minute5, T, S1T>,
    LazyTransformLast<Minute10, T, S1T>,
    LazyTransformLast<Minute30, T, S1T>,
    LazyTransformLast<Hour1, T, S1T>,
    LazyTransformLast<Hour4, T, S1T>,
    LazyTransformLast<Hour12, T, S1T>,
    LazyTransformLast<Day1, T, S1T>,
    LazyTransformLast<Day3, T, S1T>,
    LazyTransformLast<Week1, T, S1T>,
    LazyTransformLast<Month1, T, S1T>,
    LazyTransformLast<Month3, T, S1T>,
    LazyTransformLast<Month6, T, S1T>,
    LazyTransformLast<Year1, T, S1T>,
    LazyTransformLast<Year10, T, S1T>,
    LazyTransformLast<HalvingEpoch, T, S1T>,
    LazyTransformLast<DifficultyEpoch, T, S1T>,
>;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyHeightDerivedLast<T, S1T = T>(pub LazyHeightDerivedLastInner<T, S1T>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue;

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.rest.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period))
    }

    pub(crate) fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedHeightDerivedLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period))
    }

    /// Create by unary-transforming a LazyHeightDerivedLast source.
    pub(crate) fn from_lazy<F, S2T>(
        name: &str,
        version: Version,
        source: &LazyHeightDerivedLast<S1T, S2T>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S2T: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period))
    }

    /// Create by unary-transforming a LazyBinaryHeightDerivedLast source.
    pub(crate) fn from_binary<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source: &LazyBinaryHeightDerivedLast<S1T, S1aT, S1bT>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period))
    }
}
