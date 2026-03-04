use std::ops::{Add, AddAssign, Div};

use brk_types::{BasisPoints16, BasisPoints32, BasisPointsSigned16, BasisPointsSigned32, StoredF32};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, PcoVecValue, UnaryTransform};

use crate::internal::{
    Bp16ToFloat, Bp16ToPercent, Bp32ToFloat, Bp32ToPercent, Bps16ToFloat, Bps16ToPercent,
    Bps32ToFloat, Bps32ToPercent,
};

pub trait ComputedVecValue
where
    Self: PcoVecValue
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable
        + Serialize,
{
}
impl<T> ComputedVecValue for T where
    T: PcoVecValue
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable
        + Serialize
{
}

pub trait NumericValue: ComputedVecValue + From<f64> + Into<f64> {}

impl<T> NumericValue for T where T: ComputedVecValue + From<f64> + Into<f64> {}

/// Trait that associates a basis-point type with its transforms to ratio and percent.
pub trait BpsType: NumericValue + JsonSchema {
    type ToRatio: UnaryTransform<Self, StoredF32>;
    type ToPercent: UnaryTransform<Self, StoredF32>;
}

impl BpsType for BasisPoints16 {
    type ToRatio = Bp16ToFloat;
    type ToPercent = Bp16ToPercent;
}

impl BpsType for BasisPoints32 {
    type ToRatio = Bp32ToFloat;
    type ToPercent = Bp32ToPercent;
}

impl BpsType for BasisPointsSigned16 {
    type ToRatio = Bps16ToFloat;
    type ToPercent = Bps16ToPercent;
}

impl BpsType for BasisPointsSigned32 {
    type ToRatio = Bps32ToFloat;
    type ToPercent = Bps32ToPercent;
}
