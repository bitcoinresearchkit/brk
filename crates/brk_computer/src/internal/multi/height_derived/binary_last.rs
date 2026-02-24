//! Lazy binary transform for derived block with Last aggregation only.
//!
//! Newtype on `Indexes` with `LazyBinaryTransformLast` per field.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableCloneableVec};

use crate::{
    indexes_from,
    internal::{
        ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedVecValue, Indexes,
        LazyBinaryTransformLast, LazyFromHeightLast, NumericValue,
    },
};

pub type LazyBinaryHeightDerivedLastInner<T, S1T, S2T> = Indexes<
    LazyBinaryTransformLast<Minute1, T, S1T, S2T>,
    LazyBinaryTransformLast<Minute5, T, S1T, S2T>,
    LazyBinaryTransformLast<Minute10, T, S1T, S2T>,
    LazyBinaryTransformLast<Minute30, T, S1T, S2T>,
    LazyBinaryTransformLast<Hour1, T, S1T, S2T>,
    LazyBinaryTransformLast<Hour4, T, S1T, S2T>,
    LazyBinaryTransformLast<Hour12, T, S1T, S2T>,
    LazyBinaryTransformLast<Day1, T, S1T, S2T>,
    LazyBinaryTransformLast<Day3, T, S1T, S2T>,
    LazyBinaryTransformLast<Week1, T, S1T, S2T>,
    LazyBinaryTransformLast<Month1, T, S1T, S2T>,
    LazyBinaryTransformLast<Month3, T, S1T, S2T>,
    LazyBinaryTransformLast<Month6, T, S1T, S2T>,
    LazyBinaryTransformLast<Year1, T, S1T, S2T>,
    LazyBinaryTransformLast<Year10, T, S1T, S2T>,
    LazyBinaryTransformLast<HalvingEpoch, T, S1T, S2T>,
    LazyBinaryTransformLast<DifficultyEpoch, T, S1T, S2T>,
>;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyBinaryHeightDerivedLast<T, S1T = T, S2T = T>(
    pub LazyBinaryHeightDerivedLastInner<T, S1T, S2T>,
)
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue;

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyBinaryHeightDerivedLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &ComputedFromHeightSumCum<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.cumulative.read_only_boxed_clone(),
                    source2.$p.cumulative.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period))
    }

    pub(crate) fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_lazy_last::<F, _, _, _, _>(
                    name,
                    v,
                    &source1.$p,
                    &source2.$p,
                )
            };
        }

        Self(indexes_from!(period))
    }

    pub(crate) fn from_lazy_block_last_and_block_last<F, S1SourceT>(
        name: &str,
        version: Version,
        source1: &LazyFromHeightLast<S1T, S1SourceT>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2T: NumericValue,
        S1SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.read_only_boxed_clone(),
                    source2.$p.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period))
    }

    pub(crate) fn from_block_last_and_lazy_block_last<F, S2SourceT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &LazyFromHeightLast<S2T, S2SourceT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: NumericValue,
        S2SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.read_only_boxed_clone(),
                    source2.$p.read_only_boxed_clone(),
                )
            };
        }

        Self(indexes_from!(period))
    }
}
