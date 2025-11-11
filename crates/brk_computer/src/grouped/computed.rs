use std::ops::{Add, AddAssign, Div};

use vecdb::{Compressable, Formattable};

pub trait ComputedType
where
    Self: Compressable
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable,
{
}
impl<T> ComputedType for T where
    T: Compressable
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
        + Formattable
{
}
