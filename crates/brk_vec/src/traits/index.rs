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
    fn unwrap_to_usize(self) -> usize;
    fn to_usize(self) -> Result<usize>;
    fn to_string<'a>() -> &'a str;
    fn decremented(self) -> Option<Self>;
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
    #[inline]
    fn unwrap_to_usize(self) -> usize {
        self.to_usize().unwrap()
    }

    #[inline]
    fn to_usize(self) -> Result<usize> {
        self.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)
    }

    #[inline]
    fn to_string<'a>() -> &'a str {
        std::any::type_name::<I>()
    }

    #[inline]
    fn decremented(self) -> Option<Self> {
        self.unwrap_to_usize().checked_sub(1).map(Self::from)
    }
}
