//! LazyBinaryComputedFromHeightSum - block sum with lazy binary transform at height level.
//!
//! Height-level sum is lazy: `transform(source1[h], source2[h])`.
//! Day1 stats are stored since they require aggregation across heights.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom2};

use crate::{
    indexes,
    internal::{ComputedHeightDerivedSum, ComputedVecValue, NumericValue},
};

const VERSION: Version = Version::ZERO;

/// Block sum aggregation with lazy binary transform at height + computed derived indexes.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryComputedFromHeightSum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<ComputedHeightDerivedSum<T>>,
}

impl<T, S1T, S2T> LazyBinaryComputedFromHeightSum<T, S1T, S2T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: ReadableBoxedVec<Height, S1T>,
        source2: ReadableBoxedVec<Height, S2T>,
        indexes: &indexes::Vecs,
    ) -> Self {
        let v = version + VERSION;

        let height = LazyVecFrom2::transformed::<F>(name, v, source1, source2);

        let rest =
            ComputedHeightDerivedSum::forced_import(name, height.read_only_boxed_clone(), v, indexes);

        Self { height, rest: Box::new(rest) }
    }
}
