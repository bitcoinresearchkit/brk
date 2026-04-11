use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Minute10, Minute30, Month1, Month3,
    Month6, Version, Week1, Year1, Year10,
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
    pub height: LazyVecFrom1<Height, T, Height, Minute10>,
    pub minute10: LazyVecFrom1<Minute10, T, Minute10, Height>,
    pub minute30: LazyVecFrom1<Minute30, T, Minute30, Height>,
    pub hour1: LazyVecFrom1<Hour1, T, Hour1, Height>,
    pub hour4: LazyVecFrom1<Hour4, T, Hour4, Height>,
    pub hour12: LazyVecFrom1<Hour12, T, Hour12, Height>,
    pub day1: LazyVecFrom1<Day1, T, Day1, Height>,
    pub day3: LazyVecFrom1<Day3, T, Day3, Height>,
    pub week1: LazyVecFrom1<Week1, T, Week1, Height>,
    pub month1: LazyVecFrom1<Month1, T, Month1, Height>,
    pub month3: LazyVecFrom1<Month3, T, Month3, Height>,
    pub month6: LazyVecFrom1<Month6, T, Month6, Height>,
    pub year1: LazyVecFrom1<Year1, T, Year1, Height>,
    pub year10: LazyVecFrom1<Year10, T, Year10, Height>,
    pub halving: LazyVecFrom1<Halving, T, Halving, Height>,
    pub epoch: LazyVecFrom1<Epoch, T, Epoch, Height>,
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
            + UnaryTransform<Halving, T>
            + UnaryTransform<Epoch, T>,
    {
        macro_rules! period {
            ($idx:ident) => {
                LazyVecFrom1::init(
                    name,
                    version,
                    indexes.$idx.first_height.read_only_boxed_clone(),
                    |idx, _: Height| F::apply(idx),
                )
            };
        }

        Self {
            height: LazyVecFrom1::init(
                name,
                version,
                indexes.height.minute10.read_only_boxed_clone(),
                |idx, _| F::apply(idx),
            ),
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
            halving: period!(halving),
            epoch: period!(epoch),
        }
    }
}
