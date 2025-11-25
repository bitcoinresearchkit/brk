use std::ops::{Add, AddAssign, Div};

use vecdb::{Formattable, Pco};

pub trait ComputedVecValue
where
    Self: Pco
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable,
{
}
impl<T> ComputedVecValue for T where
    T: Pco
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable
{
}
