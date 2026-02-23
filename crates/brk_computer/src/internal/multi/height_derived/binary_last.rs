//! Lazy binary transform for derived block with Last aggregation only.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableCloneableVec};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedVecValue,
    LazyBinaryTransformLast, LazyFromHeightLast, NumericValue,
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryHeightDerivedLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub minute1: LazyBinaryTransformLast<Minute1, T, S1T, S2T>,
    pub minute5: LazyBinaryTransformLast<Minute5, T, S1T, S2T>,
    pub minute10: LazyBinaryTransformLast<Minute10, T, S1T, S2T>,
    pub minute30: LazyBinaryTransformLast<Minute30, T, S1T, S2T>,
    pub hour1: LazyBinaryTransformLast<Hour1, T, S1T, S2T>,
    pub hour4: LazyBinaryTransformLast<Hour4, T, S1T, S2T>,
    pub hour12: LazyBinaryTransformLast<Hour12, T, S1T, S2T>,
    pub day1: LazyBinaryTransformLast<Day1, T, S1T, S2T>,
    pub day3: LazyBinaryTransformLast<Day3, T, S1T, S2T>,
    pub week1: LazyBinaryTransformLast<Week1, T, S1T, S2T>,
    pub month1: LazyBinaryTransformLast<Month1, T, S1T, S2T>,
    pub month3: LazyBinaryTransformLast<Month3, T, S1T, S2T>,
    pub month6: LazyBinaryTransformLast<Month6, T, S1T, S2T>,
    pub year1: LazyBinaryTransformLast<Year1, T, S1T, S2T>,
    pub year10: LazyBinaryTransformLast<Year10, T, S1T, S2T>,
    pub halvingepoch: LazyBinaryTransformLast<HalvingEpoch, T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformLast<DifficultyEpoch, T, S1T, S2T>,
}

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

        Self {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        }
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

        Self {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        }
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

        Self {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        }
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

        Self {
            minute1: period!(minute1),
            minute5: period!(minute5),
            minute10: period!(minute10),
            minute30: period!(minute30),
            hour1: period!(hour1),
            hour4: period!(hour4),
            hour12: period!(hour12),
            day1: period!(day1),
            day3: period!(day3),
            week1: period!(week1),
            month1: period!(month1),
            month3: period!(month3),
            month6: period!(month6),
            year1: period!(year1),
            year10: period!(year10),
            halvingepoch: period!(halvingepoch),
            difficultyepoch: period!(difficultyepoch),
        }
    }
}
