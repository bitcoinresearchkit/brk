use std::{fmt::Debug, ops::Add};

pub trait StorableVecIndex
where
    Self: Debug + Default + Copy + Clone + TryInto<usize> + From<usize> + Add<usize, Output = Self>,
{
}
impl<I> StorableVecIndex for I where
    I: Debug + Default + Copy + Clone + TryInto<usize> + From<usize> + Add<usize, Output = Self>
{
}
