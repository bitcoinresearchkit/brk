use std::ops::{Add, Div};

pub trait StoredType
where
    Self: brk_vec::StoredType + From<usize> + Div<usize, Output = Self> + Add<Output = Self>,
{
}
impl<T> StoredType for T where
    T: brk_vec::StoredType + From<usize> + Div<usize, Output = Self> + Add<Output = Self>
{
}
