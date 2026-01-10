//! Binary transform composite from DateIndex - Last aggregation only.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSum, ComputedDateLast, ComputedVecValue, LazyBinaryTransformLast,
    LazyPeriodsLast, LazyPeriodsSumCum, NumericValue,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryDateLast<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyVecFrom2<DateIndex, T, DateIndex, S1T, DateIndex, S2T>,
    pub weekindex: LazyBinaryTransformLast<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyBinaryTransformLast<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyBinaryTransformLast<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyBinaryTransformLast<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyBinaryTransformLast<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyBinaryTransformLast<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyBinaryDateLast<T, S1T, S2T>
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_derived_last_and_computed_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &LazyPeriodsLast<S1T>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self {
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_derived_last_and_block_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &LazyPeriodsLast<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_both_derived_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &LazyPeriodsLast<S1T>,
        dateindex_source2: IterableBoxedVec<DateIndex, S2T>,
        source2: &LazyPeriodsLast<S2T>,
    ) -> Self {
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                dateindex_source2,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.0.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_dateindex_and_height_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedDateLast<S1T>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_both_block_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockLast<S1T>,
        source2: &ComputedBlockLast<S2T>,
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
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.0.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
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

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_block_last_and_height_sum<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedBlockLast<S1T>,
        source2: &ComputedBlockSum<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.0.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_both_sum_cum_cumulatives<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        dates1: &LazyPeriodsSumCum<S1T>,
        dateindex_source2: IterableBoxedVec<DateIndex, S2T>,
        dates2: &LazyPeriodsSumCum<S2T>,
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
                    dates1.$p.cumulative.boxed_clone(),
                    dates2.$p.cumulative.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                dateindex_source2,
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    /// Create from a LazyPeriodsLast source and a BinaryDateLast source.
    pub fn from_derived_last_and_binary_last<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        dateindex_source1: IterableBoxedVec<DateIndex, S1T>,
        source1: &LazyPeriodsLast<S1T>,
        source2: &LazyBinaryDateLast<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                dateindex_source1,
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    /// Create from a BinaryDateLast source and a ComputedDateLast source.
    pub fn from_binary_and_computed_last<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryDateLast<S1T, S1aT, S1bT>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    /// Create from a BinaryDateLast source and a ComputedBlockLast source.
    pub fn from_binary_and_block_last<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryDateLast<S1T, S1aT, S1bT>,
        source2: &ComputedBlockLast<S2T>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
        S2T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.0.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    /// Create from a ComputedDateLast source and a BinaryDateLast source.
    pub fn from_computed_and_binary_last<F, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &ComputedDateLast<S1T>,
        source2: &LazyBinaryDateLast<S2T, S2aT, S2bT>,
    ) -> Self
    where
        F: BinaryTransform<S1T, S2T, T>,
        S2aT: ComputedVecValue + JsonSchema,
        S2bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.boxed_clone(),
            ),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    /// Create from two BinaryDateLast sources.
    pub fn from_both_binary_last<F, S1aT, S1bT, S2aT, S2bT>(
        name: &str,
        version: Version,
        source1: &LazyBinaryDateLast<S1T, S1aT, S1bT>,
        source2: &LazyBinaryDateLast<S2T, S2aT, S2bT>,
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
                LazyBinaryTransformLast::from_vecs::<F>(
                    name,
                    v,
                    source1.$p.boxed_clone(),
                    source2.$p.boxed_clone(),
                )
            };
        }

        Self {
            dateindex: LazyVecFrom2::transformed::<F>(
                name,
                v,
                source1.dateindex.boxed_clone(),
                source2.dateindex.boxed_clone(),
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
