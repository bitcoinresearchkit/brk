//! BinaryHeightDateLast - height storage + binary transform lazy date periods.
//!
//! Use this when height is stored as EagerVec and date periods are lazy binary transforms.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, EagerVec, PcoVec};

use crate::internal::{
    ComputedDateLast, ComputedHeightDateLast, ComputedVecValue, LazyBinaryDateLast,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct BinaryHeightDateLast<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: LazyBinaryDateLast<T, S1T, S2T>,
}

impl<T, S1T, S2T> BinaryHeightDateLast<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn from_computed_both_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height: EagerVec<PcoVec<Height, T>>,
        source1: &ComputedDateLast<S1T>,
        source2: &ComputedDateLast<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height,
            rest: LazyBinaryDateLast::from_computed_both_last::<F>(name, v, source1, source2),
        }
    }

    pub fn from_computed_height_date_last<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height: EagerVec<PcoVec<Height, T>>,
        source1: &ComputedHeightDateLast<S1T>,
        source2: &ComputedHeightDateLast<S2T>,
    ) -> Self
    where
        S1T: JsonSchema + 'static,
        S2T: JsonSchema + 'static,
    {
        let v = version + VERSION;

        Self {
            height,
            rest: LazyBinaryDateLast::from_computed_both_last::<F>(
                name,
                v,
                &source1.rest,
                &source2.rest,
            ),
        }
    }
}
