use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, ReadableCloneableVec, LazyVecFrom1, UnaryTransform, VecValue};

use crate::indexes;

/// Lazy constant vecs for all index levels.
/// Uses const generic transforms to return the same value for every index.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ConstantVecs<T>
where
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    pub height: LazyVecFrom1<Height, T, Height, Height>,
    pub minute1: LazyVecFrom1<Minute1, T, Minute1, Minute1>,
    pub minute5: LazyVecFrom1<Minute5, T, Minute5, Minute5>,
    pub minute10: LazyVecFrom1<Minute10, T, Minute10, Minute10>,
    pub minute30: LazyVecFrom1<Minute30, T, Minute30, Minute30>,
    pub hour1: LazyVecFrom1<Hour1, T, Hour1, Hour1>,
    pub hour4: LazyVecFrom1<Hour4, T, Hour4, Hour4>,
    pub hour12: LazyVecFrom1<Hour12, T, Hour12, Hour12>,
    pub day1: LazyVecFrom1<Day1, T, Day1, Day1>,
    pub day3: LazyVecFrom1<Day3, T, Day3, Day3>,
    pub week1: LazyVecFrom1<Week1, T, Week1, Week1>,
    pub month1: LazyVecFrom1<Month1, T, Month1, Month1>,
    pub month3: LazyVecFrom1<Month3, T, Month3, Month3>,
    pub month6: LazyVecFrom1<Month6, T, Month6, Month6>,
    pub year1: LazyVecFrom1<Year1, T, Year1, Year1>,
    pub year10: LazyVecFrom1<Year10, T, Year10, Year10>,
    pub halvingepoch: LazyVecFrom1<HalvingEpoch, T, HalvingEpoch, HalvingEpoch>,
    pub difficultyepoch: LazyVecFrom1<DifficultyEpoch, T, DifficultyEpoch, DifficultyEpoch>,
}

impl<T: VecValue + Formattable + Serialize + JsonSchema> ConstantVecs<T> {
    /// Create constant vecs using a transform that ignores input and returns a constant.
    pub(crate) fn new<F>(name: &str, version: Version, indexes: &indexes::Vecs) -> Self
    where
        F: UnaryTransform<Height, T>
            + UnaryTransform<Minute1, T>
            + UnaryTransform<Minute5, T>
            + UnaryTransform<Minute10, T>
            + UnaryTransform<Minute30, T>
            + UnaryTransform<Hour1, T>
            + UnaryTransform<Hour4, T>
            + UnaryTransform<Hour12, T>
            + UnaryTransform<Day1, T>
            + UnaryTransform<Day3, T>
            + UnaryTransform<Week1, T>
            + UnaryTransform<Month1, T>
            + UnaryTransform<Month3, T>
            + UnaryTransform<Month6, T>
            + UnaryTransform<Year1, T>
            + UnaryTransform<Year10, T>
            + UnaryTransform<HalvingEpoch, T>
            + UnaryTransform<DifficultyEpoch, T>,
    {
        macro_rules! period {
            ($idx:ident, $I:ty) => {
                LazyVecFrom1::transformed::<F>(
                    name,
                    version,
                    indexes.$idx.identity.read_only_boxed_clone(),
                )
            };
        }

        Self {
            height: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.height.identity.read_only_boxed_clone(),
            ),
            minute1: period!(minute1, Minute1),
            minute5: period!(minute5, Minute5),
            minute10: period!(minute10, Minute10),
            minute30: period!(minute30, Minute30),
            hour1: period!(hour1, Hour1),
            hour4: period!(hour4, Hour4),
            hour12: period!(hour12, Hour12),
            day1: period!(day1, Day1),
            day3: period!(day3, Day3),
            week1: period!(week1, Week1),
            month1: period!(month1, Month1),
            month3: period!(month3, Month3),
            month6: period!(month6, Month6),
            year1: period!(year1, Year1),
            year10: period!(year10, Year10),
            halvingepoch: period!(halvingepoch, HalvingEpoch),
            difficultyepoch: period!(difficultyepoch, DifficultyEpoch),
        }
    }
}
