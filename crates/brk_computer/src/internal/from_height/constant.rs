use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour4, Hour12, Minute10, Minute30,
    Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, LazyVecFrom1, ReadableCloneableVec, UnaryTransform, VecValue};

use crate::indexes;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ConstantVecs<T>
where
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    pub height: LazyVecFrom1<Height, T, Height, Height>,
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
    pub(crate) fn new<F>(name: &str, version: Version, indexes: &indexes::Vecs) -> Self
    where
        F: UnaryTransform<Height, T>
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
            ($idx:ident) => {
                LazyVecFrom1::transformed::<F>(
                    name,
                    version,
                    indexes.$idx.identity.read_only_boxed_clone(),
                )
            };
        }

        Self {
            height: period!(height),
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
