//! Binary transform composite from DateIndex - Last aggregation only.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSum, ComputedDateLast, ComputedVecValue, DerivedDateLast,
    NumericValue,
};

use super::super::transform::LazyTransform2Last;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct BinaryDateLast<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyVecFrom2<DateIndex, T, DateIndex, S1T, DateIndex, S2T>,
    pub weekindex: LazyTransform2Last<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2Last<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2Last<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2Last<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2Last<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2Last<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> BinaryDateLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_both_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedDateLast<S1T>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }

    pub fn from_derived_last_and_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &DerivedDateLast<S1T>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                source2.dateindex.boxed_clone(),
            ),
            weekindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }

    pub fn from_both_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &DerivedDateLast<S1T>,
        dateindex_source2: IterableBoxedVec<DateIndex, S2T>,
        source2: &DerivedDateLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                dateindex_source2,
            ),
            weekindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }

    pub fn from_height_and_dateindex_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockLast<S1T>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.0.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Last::from_lazy_last::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }

    pub fn from_dateindex_last_and_height_sum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedDateLast<S1T>,
        source2: &ComputedBlockSum<S2T>,
    ) -> Self
    where
        S2T: NumericValue,
    {
        let v = version + VERSION;

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.weekindex.boxed_clone(),
                source2.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.monthindex.boxed_clone(),
                source2.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.quarterindex.boxed_clone(),
                source2.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.semesterindex.boxed_clone(),
                source2.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.yearindex.boxed_clone(),
                source2.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransform2Last::from_vecs::<F>(
                name,
                v,
                source1.decadeindex.boxed_clone(),
                source2.decadeindex.boxed_clone(),
            ),
        }
    }
}
