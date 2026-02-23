//! Lazy aggregated binary transform for Sum-only pattern across all time periods.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableCloneableVec};

use crate::internal::{
    ComputedFromHeightSumCum, ComputedHeightDerivedSum, ComputedVecValue, LazyBinaryTransformSum,
    LazyFromHeightLast, NumericValue,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryHeightDerivedSum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub minute1: LazyBinaryTransformSum<Minute1, T, S1T, S2T>,
    pub minute5: LazyBinaryTransformSum<Minute5, T, S1T, S2T>,
    pub minute10: LazyBinaryTransformSum<Minute10, T, S1T, S2T>,
    pub minute30: LazyBinaryTransformSum<Minute30, T, S1T, S2T>,
    pub hour1: LazyBinaryTransformSum<Hour1, T, S1T, S2T>,
    pub hour4: LazyBinaryTransformSum<Hour4, T, S1T, S2T>,
    pub hour12: LazyBinaryTransformSum<Hour12, T, S1T, S2T>,
    pub day1: LazyBinaryTransformSum<Day1, T, S1T, S2T>,
    pub day3: LazyBinaryTransformSum<Day3, T, S1T, S2T>,
    pub week1: LazyBinaryTransformSum<Week1, T, S1T, S2T>,
    pub month1: LazyBinaryTransformSum<Month1, T, S1T, S2T>,
    pub month3: LazyBinaryTransformSum<Month3, T, S1T, S2T>,
    pub month6: LazyBinaryTransformSum<Month6, T, S1T, S2T>,
    pub year1: LazyBinaryTransformSum<Year1, T, S1T, S2T>,
    pub year10: LazyBinaryTransformSum<Year10, T, S1T, S2T>,
    pub halvingepoch: LazyBinaryTransformSum<HalvingEpoch, T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformSum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryHeightDerivedSum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub(crate) fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedSum<S1T>,
        source2: &ComputedHeightDerivedSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSum::from_boxed::<F>(
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

    /// Create from two LazyBinaryHeightDerivedSum sources.
    pub(crate) fn from_binary<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryHeightDerivedSum<S1T, S1aT, S1bT>,
        source2: &LazyBinaryHeightDerivedSum<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSum::from_boxed::<F>(
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

    /// Create from a SumCum source (using only sum) and a LazyLast source.
    pub(crate) fn from_sumcum_lazy_last<F, S2ST>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &LazyFromHeightLast<S2T, S2ST>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2ST: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSum::from_boxed::<F>(
                    name,
                    v,
                    source1.$p.sum.read_only_boxed_clone(),
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
