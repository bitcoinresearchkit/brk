use std::ops::{Add, AddAssign, Div};

use vecdb::{Formattable, PcoVecValue};

pub trait ComputedVecValue
where
    Self: PcoVecValue
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable,
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
{
}
