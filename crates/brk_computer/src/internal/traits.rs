//! Internal traits for computed vec values.

use std::ops::{Add, AddAssign, Div};

use serde::Serialize;
use vecdb::{Formattable, PcoVecValue};

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
