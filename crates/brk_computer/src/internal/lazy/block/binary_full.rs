//! Lazy binary transform from Full sources.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, LazyVecFrom2};

use crate::internal::{ComputedBlockFull, ComputedVecValue, DerivedTxFull, NumericValue};

use super::super::derived_block::LazyDerivedBlock2SumCum;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct BinaryBlockFull<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(wrap = "base")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: LazyDerivedBlock2SumCum<T, S1T, S2T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> BinaryBlockFull<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_height_and_txindex<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedBlockFull<S1T>,
        source2: &DerivedTxFull<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            rest: LazyDerivedBlock2SumCum::from_derived_full::<F, _, _, _, _>(
                name,
                v,
                &source1.dateindex.sum_cum,
                &source1.rest,
                &source1.difficultyepoch,
                &source2.dateindex.sum_cum,
                &source2.dates,
                &source2.difficultyepoch,
            ),
        }
    }
}
