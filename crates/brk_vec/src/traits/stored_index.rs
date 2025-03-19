use std::{fmt::Debug, ops::Add};

use crate::{Error, Result};

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
    fn to_usize(self) -> Result<usize>;
}
impl<I> StoredIndex for I
where
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
        + Sync,
{
    #[inline(always)]
    fn to_usize(self) -> Result<usize> {
        self.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)
    }
}
