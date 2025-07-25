use std::ops::{Add, AddAssign, Div};

use brk_vecs::StoredType;

pub trait ComputedType
where
    Self:
        StoredType + From<usize> + Div<usize, Output = Self> + Add<Output = Self> + AddAssign + Ord,
{
}
impl<T> ComputedType for T where
    T: StoredType + From<usize> + Div<usize, Output = Self> + Add<Output = Self> + AddAssign + Ord
{
}
