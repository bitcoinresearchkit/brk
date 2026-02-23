//! Lazy aggregated Last for block-level sources.

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Hour1, Hour12, Hour4, Minute1, Minute10, Minute30,
    Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::internal::{
    ComputedFromHeightLast, ComputedHeightDerivedLast, ComputedVecValue,
    LazyBinaryHeightDerivedLast, LazyTransformLast, NumericValue,
};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyHeightDerivedLast<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub minute1: LazyTransformLast<Minute1, T, S1T>,
    pub minute5: LazyTransformLast<Minute5, T, S1T>,
    pub minute10: LazyTransformLast<Minute10, T, S1T>,
    pub minute30: LazyTransformLast<Minute30, T, S1T>,
    pub hour1: LazyTransformLast<Hour1, T, S1T>,
    pub hour4: LazyTransformLast<Hour4, T, S1T>,
    pub hour12: LazyTransformLast<Hour12, T, S1T>,
    pub day1: LazyTransformLast<Day1, T, S1T>,
    pub day3: LazyTransformLast<Day3, T, S1T>,
    pub week1: LazyTransformLast<Week1, T, S1T>,
    pub month1: LazyTransformLast<Month1, T, S1T>,
    pub month3: LazyTransformLast<Month3, T, S1T>,
    pub month6: LazyTransformLast<Month6, T, S1T>,
    pub year1: LazyTransformLast<Year1, T, S1T>,
    pub year10: LazyTransformLast<Year10, T, S1T>,
    pub halvingepoch: LazyTransformLast<HalvingEpoch, T, S1T>,
    pub difficultyepoch: LazyTransformLast<DifficultyEpoch, T, S1T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T> LazyHeightDerivedLast<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.rest.$p.read_only_boxed_clone())
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

    pub(crate) fn from_derived_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedHeightDerivedLast<S1T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
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

    /// Create by unary-transforming a LazyHeightDerivedLast source.
    pub(crate) fn from_lazy<F, S2T>(
        name: &str,
        version: Version,
        source: &LazyHeightDerivedLast<S1T, S2T>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S2T: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
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

    /// Create by unary-transforming a LazyBinaryHeightDerivedLast source.
    pub(crate) fn from_binary<F, S1aT, S1bT>(
        name: &str,
        version: Version,
        source: &LazyBinaryHeightDerivedLast<S1T, S1aT, S1bT>,
    ) -> Self
    where
        F: UnaryTransform<S1T, T>,
        S1aT: ComputedVecValue + JsonSchema,
        S1bT: ComputedVecValue + JsonSchema,
    {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_boxed::<F>(name, v, source.$p.read_only_boxed_clone())
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
