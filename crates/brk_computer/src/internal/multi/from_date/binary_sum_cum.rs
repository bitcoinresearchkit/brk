//! Binary transform for SumCum pattern across date periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedHeightDerivedLast,
    ComputedHeightDerivedSumCum, ComputedVecValue, LazyBinaryTransformSumCum, LazyDateDerivedFull,
    LazyDateDerivedSumCum, LazyFromHeightLast, NumericValue, SumCum,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryFromDateSumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyBinaryTransformSumCum<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyBinaryTransformSumCum<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyBinaryTransformSumCum<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyBinaryTransformSumCum<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyBinaryTransformSumCum<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyBinaryTransformSumCum<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyBinaryTransformSumCum<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryFromDateSumCum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<DateIndex, S1T>,
        periods1: &LazyDateDerivedSumCum<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        periods2: &LazyDateDerivedSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources::<F>(
                    name, v,
                    periods1.$p.sum.boxed_clone(), periods2.$p.sum.boxed_clone(),
                    periods1.$p.cumulative.boxed_clone(), periods2.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_derived_full<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<DateIndex, S1T>,
        dates1: &LazyDateDerivedFull<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        dates2: &LazyDateDerivedFull<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                    name, v, &dates1.$p, &dates2.$p,
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    // --- Raw variants (no _sum suffix) for pure SumCum types ---

    #[allow(clippy::too_many_arguments)]
    pub fn from_computed_sum_raw<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex1: &SumCum<DateIndex, S1T>,
        periods1: &LazyDateDerivedSumCum<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        periods2: &LazyDateDerivedSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_sum_raw::<F>(
                    name, v,
                    periods1.$p.sum.boxed_clone(), periods2.$p.sum.boxed_clone(),
                    periods1.$p.cumulative.boxed_clone(), periods2.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum_sum_raw::<F>(name, v, dateindex1, dateindex2),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    // --- Methods accepting SumCum + Last sources ---

    pub fn from_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.rest.$p.sum.boxed_clone(),
                    source1.rest.$p.cumulative.boxed_clone(),
                    source2.rest.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum_last_sum_raw::<F>(
                name, v, &source1.dateindex, &source2.dateindex,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_derived_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedSumCum<S1T>,
        source2: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.$p.sum.boxed_clone(),
                    source1.$p.cumulative.boxed_clone(),
                    source2.rest.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum_last_sum_raw::<F>(
                name, v, &source1.dateindex, &source2.dateindex,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_computed_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightSumCum<S1T>,
        source2: &ComputedHeightDerivedLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.rest.$p.sum.boxed_clone(),
                    source1.rest.$p.cumulative.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum_last_sum_raw::<F>(
                name, v, &source1.dateindex, &source2.dateindex,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedHeightDerivedSumCum<S1T>,
        source2: &ComputedHeightDerivedLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.$p.sum.boxed_clone(),
                    source1.$p.cumulative.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sum_cum_last_sum_raw::<F>(
                name, v, &source1.dateindex, &source2.dateindex,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    // --- Methods accepting SumCum + LazyLast sources ---

    pub fn from_computed_lazy_last<F, S2ST>(
        name: &str,
        version: Version,
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

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.rest.$p.sum.boxed_clone(),
                    source1.rest.$p.cumulative.boxed_clone(),
                    source2.rest.dates.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyBinaryTransformSumCum::from_sources_last_sum_raw::<F>(
                name, v,
                source1.dateindex.boxed_sum(),
                source1.dateindex.boxed_cumulative(),
                source2.rest.dates.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }
}
