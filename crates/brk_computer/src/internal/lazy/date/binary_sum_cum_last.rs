//! Binary transform for SumCum + Last pattern across date periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSumCum, ComputedVecValue, DerivedComputedBlockLast,
    DerivedComputedBlockSumCum, NumericValue,
};

use super::super::transform::LazyTransform2SumCumLast;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDate2SumCumLast<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyTransform2SumCumLast<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyTransform2SumCumLast<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2SumCumLast<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2SumCumLast<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2SumCumLast<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2SumCumLast<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2SumCumLast<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDate2SumCumLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
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
            dateindex: LazyTransform2SumCumLast::from_sources::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.weekindex.sum.boxed_clone(),
                source1.rest.weekindex.cumulative.boxed_clone(),
                source2.rest.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.monthindex.sum.boxed_clone(),
                source1.rest.monthindex.cumulative.boxed_clone(),
                source2.rest.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.quarterindex.sum.boxed_clone(),
                source1.rest.quarterindex.cumulative.boxed_clone(),
                source2.rest.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.semesterindex.sum.boxed_clone(),
                source1.rest.semesterindex.cumulative.boxed_clone(),
                source2.rest.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.yearindex.sum.boxed_clone(),
                source1.rest.yearindex.cumulative.boxed_clone(),
                source2.rest.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.decadeindex.sum.boxed_clone(),
                source1.rest.decadeindex.cumulative.boxed_clone(),
                source2.rest.decadeindex.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed_full<F: BinaryTransform<S1T, S2T, T>>(
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
            dateindex: LazyTransform2SumCumLast::from_sources::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.weekindex.sum.boxed_clone(),
                source1.weekindex.cumulative.boxed_clone(),
                source2.rest.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.monthindex.sum.boxed_clone(),
                source1.monthindex.cumulative.boxed_clone(),
                source2.rest.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.quarterindex.sum.boxed_clone(),
                source1.quarterindex.cumulative.boxed_clone(),
                source2.rest.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.semesterindex.sum.boxed_clone(),
                source1.semesterindex.cumulative.boxed_clone(),
                source2.rest.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.yearindex.sum.boxed_clone(),
                source1.yearindex.cumulative.boxed_clone(),
                source2.rest.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.decadeindex.sum.boxed_clone(),
                source1.decadeindex.cumulative.boxed_clone(),
                source2.rest.decadeindex.boxed_clone(),
            ),
        }
    }

    pub fn from_computed_derived_computed<F: BinaryTransform<S1T, S2T, T>>(
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
            dateindex: LazyTransform2SumCumLast::from_sources::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.weekindex.sum.boxed_clone(),
                source1.rest.weekindex.cumulative.boxed_clone(),
                source2.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.monthindex.sum.boxed_clone(),
                source1.rest.monthindex.cumulative.boxed_clone(),
                source2.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.quarterindex.sum.boxed_clone(),
                source1.rest.quarterindex.cumulative.boxed_clone(),
                source2.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.semesterindex.sum.boxed_clone(),
                source1.rest.semesterindex.cumulative.boxed_clone(),
                source2.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.yearindex.sum.boxed_clone(),
                source1.rest.yearindex.cumulative.boxed_clone(),
                source2.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.rest.decadeindex.sum.boxed_clone(),
                source1.rest.decadeindex.cumulative.boxed_clone(),
                source2.decadeindex.boxed_clone(),
            ),
        }
    }

    pub fn from_derived_computed<F: BinaryTransform<S1T, S2T, T>>(
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
            dateindex: LazyTransform2SumCumLast::from_sources::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.weekindex.sum.boxed_clone(),
                source1.weekindex.cumulative.boxed_clone(),
                source2.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.monthindex.sum.boxed_clone(),
                source1.monthindex.cumulative.boxed_clone(),
                source2.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.quarterindex.sum.boxed_clone(),
                source1.quarterindex.cumulative.boxed_clone(),
                source2.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.semesterindex.sum.boxed_clone(),
                source1.semesterindex.cumulative.boxed_clone(),
                source2.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.yearindex.sum.boxed_clone(),
                source1.yearindex.cumulative.boxed_clone(),
                source2.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2SumCumLast::from_boxed::<F>(
                name,
                v,
                source1.decadeindex.sum.boxed_clone(),
                source1.decadeindex.cumulative.boxed_clone(),
                source2.decadeindex.boxed_clone(),
            ),
        }
    }
}
