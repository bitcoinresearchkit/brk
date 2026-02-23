//! Lazy aggregated SumCum - binary transform version.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour4, Hour12, Minute1, Minute5, Minute10,
    Minute30, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableCloneableVec};

use crate::internal::{
    ComputedFromHeightSumCum, ComputedHeightDerivedFull, ComputedHeightDerivedSumCum,
    ComputedVecValue, LazyBinaryTransformSumCum, LazyFromHeightLast, NumericValue, TxDerivedFull,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryHeightDerivedSumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub minute1: LazyBinaryTransformSumCum<Minute1, T, S1T, S2T>,
    pub minute5: LazyBinaryTransformSumCum<Minute5, T, S1T, S2T>,
    pub minute10: LazyBinaryTransformSumCum<Minute10, T, S1T, S2T>,
    pub minute30: LazyBinaryTransformSumCum<Minute30, T, S1T, S2T>,
    pub hour1: LazyBinaryTransformSumCum<Hour1, T, S1T, S2T>,
    pub hour4: LazyBinaryTransformSumCum<Hour4, T, S1T, S2T>,
    pub hour12: LazyBinaryTransformSumCum<Hour12, T, S1T, S2T>,
    pub day1: LazyBinaryTransformSumCum<Day1, T, S1T, S2T>,
    pub day3: LazyBinaryTransformSumCum<Day3, T, S1T, S2T>,
    pub week1: LazyBinaryTransformSumCum<Week1, T, S1T, S2T>,
    pub month1: LazyBinaryTransformSumCum<Month1, T, S1T, S2T>,
    pub month3: LazyBinaryTransformSumCum<Month3, T, S1T, S2T>,
    pub month6: LazyBinaryTransformSumCum<Month6, T, S1T, S2T>,
    pub year1: LazyBinaryTransformSumCum<Year1, T, S1T, S2T>,
    pub year10: LazyBinaryTransformSumCum<Year10, T, S1T, S2T>,
    pub halvingepoch: LazyBinaryTransformSumCum<HalvingEpoch, T, S1T, S2T>,
    pub difficultyepoch: LazyBinaryTransformSumCum<DifficultyEpoch, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryHeightDerivedSumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create from two ComputedHeightDerivedSumCum sources.
    pub(crate) fn from_computed_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedSumCum<S1T>,
        source2: &ComputedHeightDerivedSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_sum_raw::<F>(
                    name,
                    v,
                    source1.$p.sum.read_only_boxed_clone(),
                    source2.$p.sum.read_only_boxed_clone(),
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

    /// Create from ComputedHeightDerivedFull + TxDerivedFull sources.
    pub(crate) fn from_full_sources<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedFull<S1T>,
        source2: &TxDerivedFull<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: PartialOrd,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                    name, v, &source1.$p, &source2.$p,
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

    // --- Methods accepting SumCum + LazyLast sources ---

    pub(crate) fn from_computed_lazy_last<F, S2ST>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &LazyFromHeightLast<S2T, S2ST>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1T: PartialOrd,
        S2T: NumericValue,
        S2ST: ComputedVecValue + schemars::JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name,
                    v,
                    source1.$p.sum.read_only_boxed_clone(),
                    source1.$p.cumulative.read_only_boxed_clone(),
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
