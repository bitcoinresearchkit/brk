use std::ops::{Add, AddAssign, Div};

use brk_vecs::StoredCompressed;

pub trait ComputedType
where
    Self: StoredCompressed
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord,
{
}
impl<T> ComputedType for T where
    T: StoredCompressed
        + From<usize>
        + Div<usize, Output = Self>
        + Add<Output = Self>
        + AddAssign
        + Ord
{
}
