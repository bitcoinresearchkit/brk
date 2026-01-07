//! Binary transform for Sum-only pattern across date periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec};

use crate::internal::{ComputedVecValue, DerivedComputedBlockSum, LazyTransform2Sum, NumericValue};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDate2Sum<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyTransform2Sum<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyTransform2Sum<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2Sum<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2Sum<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2Sum<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2Sum<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2Sum<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyDate2Sum<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn from_derived<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &DerivedComputedBlockSum<S1T>,
        source2: &DerivedComputedBlockSum<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyTransform2Sum::from_sum::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.weekindex.boxed_clone(),
                source2.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.monthindex.boxed_clone(),
                source2.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.quarterindex.boxed_clone(),
                source2.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.semesterindex.boxed_clone(),
                source2.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.yearindex.boxed_clone(),
                source2.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2Sum::from_boxed::<F>(
                name,
                v,
                source1.decadeindex.boxed_clone(),
                source2.decadeindex.boxed_clone(),
            ),
        }
    }
}
