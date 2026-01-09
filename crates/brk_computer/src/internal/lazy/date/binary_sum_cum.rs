//! Binary transform for SumCum pattern across date periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedVecValue, DerivedComputedBlockLast,
    DerivedComputedBlockSumCum, DerivedDateFull, DerivedDateSumCum, NumericValue, SumCum,
};

use super::super::transform::LazyTransform2SumCum;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDate2SumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyTransform2SumCum<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyTransform2SumCum<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2SumCum<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2SumCum<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2SumCum<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2SumCum<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2SumCum<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDate2SumCum<T, S1T, S2T>
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
        periods1: &DerivedDateSumCum<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        periods2: &DerivedDateSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources::<F>(
                    name, v,
                    periods1.$p.sum.boxed_clone(), periods2.$p.sum.boxed_clone(),
                    periods1.$p.cumulative.boxed_clone(), periods2.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
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
        dates1: &DerivedDateFull<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        dates2: &DerivedDateFull<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_lazy_stats_aggregate::<F, _, _, _, _>(
                    name, v, &dates1.$p, &dates2.$p,
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum::<F>(name, v, dateindex1, dateindex2),
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
        periods1: &DerivedDateSumCum<S1T>,
        dateindex2: &SumCum<DateIndex, S2T>,
        periods2: &DerivedDateSumCum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources_sum_raw::<F>(
                    name, v,
                    periods1.$p.sum.boxed_clone(), periods2.$p.sum.boxed_clone(),
                    periods1.$p.cumulative.boxed_clone(), periods2.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum_sum_raw::<F>(name, v, dateindex1, dateindex2),
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
        source1: &ComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.rest.$p.sum.boxed_clone(),
                    source1.rest.$p.cumulative.boxed_clone(),
                    source2.rest.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum_last_sum_raw::<F>(
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
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.$p.sum.boxed_clone(),
                    source1.$p.cumulative.boxed_clone(),
                    source2.rest.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum_last_sum_raw::<F>(
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
        source1: &ComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: PartialOrd,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.rest.$p.sum.boxed_clone(),
                    source1.rest.$p.cumulative.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum_last_sum_raw::<F>(
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
        source1: &DerivedComputedBlockSumCum<S1T>,
        source2: &DerivedComputedBlockLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransform2SumCum::from_sources_last_sum_raw::<F>(
                    name, v,
                    source1.$p.sum.boxed_clone(),
                    source1.$p.cumulative.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyTransform2SumCum::from_sum_cum_last_sum_raw::<F>(
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
}
