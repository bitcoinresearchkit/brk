use std::{fmt::Debug, ops::Add};

pub trait StoredIndex
where
    Self: Debug
        + Default
        + Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + TryInto<usize>
        + From<usize>
        + Add<usize, Output = Self>
        + Send
        + Sync,
{
}
impl<I> StoredIndex for I where
    I: Debug
        + Default
        + Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + TryInto<usize>
        + From<usize>
        + Add<usize, Output = Self>
        + Send
        + Sync
{
}
