//! Lazy transform for Last-only date sources.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableBoxedVec, IterableCloneableVec, UnaryTransform};

use crate::internal::{ComputedBlockLast, ComputedDateLast, ComputedVecValue, DerivedDateLast, NumericValue};

use super::super::transform::LazyTransformLast;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDateLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: LazyTransformLast<DateIndex, T, S1T>,
    pub weekindex: LazyTransformLast<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformLast<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformLast<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformLast<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformLast<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformLast<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyDateLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub fn from_source<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedDateLast<S1T>,
    ) -> Self {
        Self::from_computed::<F>(name, version, source.dateindex.boxed_clone(), source)
    }

    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, S1T>,
        source: &ComputedDateLast<S1T>,
    ) -> Self {
        Self::from_derived::<F>(name, version, dateindex_source, &source.rest)
    }

    pub fn from_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex_source: IterableBoxedVec<DateIndex, S1T>,
        source: &DerivedDateLast<S1T>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_lazy_last::<F, _, _>(name, v, &source.$p)
            };
        }

        Self {
            dateindex: LazyTransformLast::from_boxed::<F>(name, v, dateindex_source),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
        }
    }

    pub fn from_block_source<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedBlockLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        Self::from_derived::<F>(name, version, source.dateindex.0.boxed_clone(), &source.dates)
    }
}
