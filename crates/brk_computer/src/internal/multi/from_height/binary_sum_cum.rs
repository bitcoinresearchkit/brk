//! Lazy binary transform from two SumCum sources.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedFromHeightSumCum, ComputedHeightDerivedSumCum,
    ComputedVecValue, LazyBinaryHeightDerivedSumCum, LazyFromHeightLast, NumericValue,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryFromHeightSumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[traversable(rename = "cumulative")]
    pub height_cumulative: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: Box<LazyBinaryHeightDerivedSumCum<T, S1T, S2T>>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyBinaryFromHeightSumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: ReadableBoxedVec<Height, S1T>,
        height_source2: ReadableBoxedVec<Height, S2T>,
        source1: &ComputedHeightDerivedSumCum<S1T>,
        source2: &ComputedHeightDerivedSumCum<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.read_only_boxed_clone(),
                source2.height_cumulative.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedSumCum::from_computed_sum_raw::<F>(
                name,
                v,
                source1,
                source2,
            )),
        }
    }

    // --- Methods accepting SumCum + LazyLast sources ---

    pub(crate) fn from_computed_lazy_last<F, S2ST>(
        name: &str,
        version: Version,
        height_source1: ReadableBoxedVec<Height, S1T>,
        height_source2: ReadableBoxedVec<Height, S2T>,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &LazyFromHeightLast<S2T, S2ST>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: PartialOrd,
        S2T: NumericValue,
        S2ST: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_cumulative: LazyVecFrom2::transformed::<F>(
                &format!("{name}_cumulative"),
                v,
                source1.height_cumulative.read_only_boxed_clone(),
                source2.height.read_only_boxed_clone(),
            ),
            rest: Box::new(LazyBinaryHeightDerivedSumCum::from_computed_lazy_last::<F, S2ST>(name, v, source1, source2)),
        }
    }
}
