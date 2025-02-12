use std::ops::Add;

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct Dateindex(u16);

impl From<Dateindex> for usize {
    fn from(value: Dateindex) -> Self {
        value.0 as usize
    }
}

impl From<usize> for Dateindex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl Add<usize> for Dateindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}
