//! LazyHeightDerivedLast — unary transform of height-derived last values.

use std::marker::PhantomData;

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour4, Hour12, Minute1, Minute5,
    Minute10, Minute30, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom1, ReadableBoxedVec, ReadableCloneableVec, UnaryTransform, VecIndex, VecValue};

use crate::{
    indexes, indexes_from,
    internal::{
        ComputedFromHeightLast, ComputedHeightDerivedLast, ComputedVecValue, Indexes, NumericValue,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyTransformLast<I, T, S1T = T>(pub LazyVecFrom1<I, T, I, S1T>)
where
    I: VecIndex,
    T: VecValue + PartialOrd + JsonSchema,
    S1T: VecValue;

impl<I, T, S1T> LazyTransformLast<I, T, S1T>
where
    I: VecIndex,
    T: VecValue + PartialOrd + JsonSchema + 'static,
    S1T: VecValue + JsonSchema,
{
    fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self(LazyVecFrom1::transformed::<F>(name, version, source))
    }
}

struct MapOption<F>(PhantomData<F>);

impl<F, S, T> UnaryTransform<Option<S>, Option<T>> for MapOption<F>
where
    F: UnaryTransform<S, T>,
{
    #[inline(always)]
    fn apply(value: Option<S>) -> Option<T> {
        value.map(F::apply)
    }
}

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyHeightDerivedLast<T, S1T = T>(
    #[allow(clippy::type_complexity)]
    pub  Indexes<
        LazyTransformLast<Minute1, Option<T>, Option<S1T>>,
        LazyTransformLast<Minute5, Option<T>, Option<S1T>>,
        LazyTransformLast<Minute10, Option<T>, Option<S1T>>,
        LazyTransformLast<Minute30, Option<T>, Option<S1T>>,
        LazyTransformLast<Hour1, Option<T>, Option<S1T>>,
        LazyTransformLast<Hour4, Option<T>, Option<S1T>>,
        LazyTransformLast<Hour12, Option<T>, Option<S1T>>,
        LazyTransformLast<Day1, Option<T>, Option<S1T>>,
        LazyTransformLast<Day3, Option<T>, Option<S1T>>,
        LazyTransformLast<Week1, Option<T>, Option<S1T>>,
        LazyTransformLast<Month1, Option<T>, Option<S1T>>,
        LazyTransformLast<Month3, Option<T>, Option<S1T>>,
        LazyTransformLast<Month6, Option<T>, Option<S1T>>,
        LazyTransformLast<Year1, Option<T>, Option<S1T>>,
        LazyTransformLast<Year10, Option<T>, Option<S1T>>,
        LazyTransformLast<HalvingEpoch, T, S1T>,
        LazyTransformLast<DifficultyEpoch, T, S1T>,
    >,
)
where
    T: VecValue + PartialOrd + JsonSchema,
    S1T: VecValue;

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedLast<T, S1T>
where
    T: VecValue + PartialOrd + JsonSchema + 'static,
    S1T: VecValue + PartialOrd + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        Self::from_derived_computed::<F>(name, version, &source.rest)
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
        let derived = ComputedHeightDerivedLast::forced_import(name, height_source, version, indexes);
        Self::from_derived_computed::<F>(name, version, &derived)
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
                LazyTransformLast::from_boxed::<MapOption<F>>(
                    name,
                    v,
                    source.$p.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period, epoch))
    }

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
                LazyTransformLast::from_boxed::<MapOption<F>>(
                    name,
                    v,
                    source.$p.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
            };
        }

        Self(indexes_from!(period, epoch))
    }
}
