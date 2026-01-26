//! Unary transform composite from Height+Date - Last aggregation only.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{ComputedFromHeightAndDateLast, ComputedVecValue, LazyTransformLast};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyUnaryFromHeightAndDateLast<T, ST>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    ST: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, ST>,
    pub dateindex: LazyTransformLast<DateIndex, T, ST>,
    pub weekindex: LazyTransformLast<WeekIndex, T, ST>,
    pub monthindex: LazyTransformLast<MonthIndex, T, ST>,
    pub quarterindex: LazyTransformLast<QuarterIndex, T, ST>,
    pub semesterindex: LazyTransformLast<SemesterIndex, T, ST>,
    pub yearindex: LazyTransformLast<YearIndex, T, ST>,
    pub decadeindex: LazyTransformLast<DecadeIndex, T, ST>,
    pub difficultyepoch: LazyTransformLast<DifficultyEpoch, T, ST>,
}

impl<T, ST> LazyUnaryFromHeightAndDateLast<T, ST>
where
    T: ComputedVecValue + JsonSchema + 'static,
    ST: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_last<F: UnaryTransform<ST, T>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightAndDateLast<ST>,
    ) -> Self {
        let v = version + VERSION;

        macro_rules! period {
            ($p:ident) => {
                LazyTransformLast::from_lazy_last::<F, _, _>(name, v, &source.rest.$p)
            };
        }

        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, source.height.boxed_clone()),
            dateindex: LazyTransformLast(LazyVecFrom1::transformed::<F>(
                name,
                v,
                source.rest.dateindex.boxed_clone(),
            )),
            weekindex: period!(weekindex),
            monthindex: period!(monthindex),
            quarterindex: period!(quarterindex),
            semesterindex: period!(semesterindex),
            yearindex: period!(yearindex),
            decadeindex: period!(decadeindex),
            difficultyepoch: LazyTransformLast::from_lazy_last::<F, _, _>(
                name,
                v,
                &source.difficultyepoch,
            ),
        }
    }
}
