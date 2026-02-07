//! Lazy binary transform for derived block with Last aggregation only.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedFromHeightAndDateLast, ComputedVecValue,
    LazyBinaryFromDateLast, LazyBinaryTransformLast, LazyFromHeightLast, NumericValue,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryHeightDerivedLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyBinaryFromDateLast<T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformLast<DifficultyEpoch, T, S1T, S2T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyBinaryHeightDerivedLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_sum_cum<F: BinaryTransform<S1T, S2T, T>>(
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

        Self {
            dates: LazyBinaryFromDateLast::from_both_sum_cum_cumulatives::<F>(
                name,
                v,
                source1.dateindex.cumulative.boxed_clone(),
                &source1.dates,
                source2.dateindex.cumulative.boxed_clone(),
                &source2.dates,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.cumulative.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
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

        Self {
            dates: LazyBinaryFromDateLast::from_both_block_last::<F>(name, v, source1, source2),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_lazy_block_last_and_block_last<F, S1SourceT>(
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

        Self {
            dates: LazyBinaryFromDateLast::from_lazy_block_last_and_block_last::<F, _>(
                name, v, source1, source2,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.rest.difficultyepoch.boxed_clone(),
                source2.rest.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_height_date_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &ComputedFromHeightAndDateLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryFromDateLast::from_computed_both_last::<F>(
                name,
                v,
                &source1.rest,
                &source2.rest,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_block_last_and_lazy_block_last<F, S2SourceT>(
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

        Self {
            dates: LazyBinaryFromDateLast::from_block_last_and_lazy_block_last::<F, _>(
                name, v, source1, source2,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.rest.difficultyepoch.boxed_clone(),
                source2.rest.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_height_date_and_block_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryFromDateLast::from_dateindex_and_height_last::<F>(
                name,
                v,
                &source1.rest,
                source2,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    /// Create from a ComputedFromHeightAndDateLast and a LazyFromHeightLast.
    pub fn from_computed_height_date_and_lazy_block_last<F, S2SourceT>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightAndDateLast<S1T>,
        source2: &LazyFromHeightLast<S2T, S2SourceT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: PartialOrd,
        S2SourceT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        Self {
            dates: LazyBinaryFromDateLast::from_computed_and_lazy_last::<F, _>(
                name,
                v,
                &source1.rest,
                &source2.rest.dates,
            ),
            difficultyepoch: LazyBinaryTransformLast::from_vecs::<F>(
                name,
                v,
                source1.difficultyepoch.boxed_clone(),
                source2.rest.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
