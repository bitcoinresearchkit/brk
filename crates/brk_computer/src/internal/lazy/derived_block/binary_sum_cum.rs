//! Lazy aggregated SumCum - binary transform version.

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedVecValue, DerivedComputedBlockLast,
    DerivedComputedBlockSumCum, DerivedDateFull, DerivedDateSumCum, LazyDate2SumCum, LazyFull,
    LazySumCum, NumericValue, SumCum,
};

use super::super::transform::LazyTransform2SumCum;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyDerivedBlock2SumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    pub dates: LazyDate2SumCum<T, S1T, S2T>,
    pub difficultyepoch: LazyTransform2SumCum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDerivedBlock2SumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<brk_types::DateIndex, S1T>,
        periods1: &DerivedDateSumCum<S1T>,
        difficultyepoch1: &LazySumCum<DifficultyEpoch, S1T, Height, DifficultyEpoch>,
        dateindex2: &SumCum<brk_types::DateIndex, S2T>,
        periods2: &DerivedDateSumCum<S2T>,
        difficultyepoch2: &LazySumCum<DifficultyEpoch, S2T, Height, DifficultyEpoch>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_computed::<F>(
                name, v, dateindex1, periods1, dateindex2, periods2,
            ),
            difficultyepoch: LazyTransform2SumCum::from_sources::<F>(
                name,
                v,
                difficultyepoch1.sum.boxed_clone(),
                difficultyepoch2.sum.boxed_clone(),
                difficultyepoch1.cumulative.boxed_clone(),
                difficultyepoch2.cumulative.boxed_clone(),
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_derived_full<F, S1I, S1L, S2I, S2L>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<brk_types::DateIndex, S1T>,
        dates1: &DerivedDateFull<S1T>,
        difficultyepoch1: &LazyFull<DifficultyEpoch, S1T, S1I, S1L>,
        dateindex2: &SumCum<brk_types::DateIndex, S2T>,
        dates2: &DerivedDateFull<S2T>,
        difficultyepoch2: &LazyFull<DifficultyEpoch, S2T, S2I, S2L>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1I: vecdb::VecIndex + 'static,
        S1L: ComputedVecValue,
        S2I: vecdb::VecIndex + 'static,
        S2L: ComputedVecValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_derived_full::<F>(
                name, v, dateindex1, dates1, dateindex2, dates2,
            ),
            difficultyepoch: LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                name,
                v,
                difficultyepoch1,
                difficultyepoch2,
            ),
        }
    }

    /// Without _sum suffix for pure SumCum types.
    #[allow(clippy::too_many_arguments)]
    pub fn from_computed_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<brk_types::DateIndex, S1T>,
        periods1: &DerivedDateSumCum<S1T>,
        difficultyepoch1: &LazySumCum<DifficultyEpoch, S1T, Height, DifficultyEpoch>,
        dateindex2: &SumCum<brk_types::DateIndex, S2T>,
        periods2: &DerivedDateSumCum<S2T>,
        difficultyepoch2: &LazySumCum<DifficultyEpoch, S2T, Height, DifficultyEpoch>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_computed_sum_raw::<F>(
                name, v, dateindex1, periods1, dateindex2, periods2,
            ),
            difficultyepoch: LazyTransform2SumCum::from_sources_sum_raw::<F>(
                name,
                v,
                difficultyepoch1.sum.boxed_clone(),
                difficultyepoch2.sum.boxed_clone(),
                difficultyepoch1.cumulative.boxed_clone(),
                difficultyepoch2.cumulative.boxed_clone(),
            ),
        }
    }

    // --- Methods accepting SumCum + Last sources ---

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_computed_last::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_derived_computed_last::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_computed_derived_last::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dates: LazyDate2SumCum::from_derived_last::<F>(name, v, source1, source2),
            difficultyepoch: LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                name,
                v,
                source1.difficultyepoch.sum.boxed_clone(),
                source1.difficultyepoch.cumulative.boxed_clone(),
                source2.difficultyepoch.boxed_clone(),
            ),
        }
    }
}
