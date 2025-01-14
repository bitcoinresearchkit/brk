use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txindex(u32);

impl Txindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }
}

impl Add<Txindex> for Txindex {
    type Output = Self;
    fn add(self, rhs: Txindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Txindex> for Txindex {
    fn add_assign(&mut self, rhs: Txindex) {
        self.0 += rhs.0
    }
}

impl From<u32> for Txindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Txindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Txindex> for u64 {
    fn from(value: Txindex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for Txindex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Txindex> for usize {
    fn from(value: Txindex) -> Self {
        value.0 as usize
    }
}

impl TryFrom<Slice> for Txindex {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}
impl TryFrom<&[u8]> for Txindex {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from(value.read_be_u32()?))
    }
}
impl From<Txindex> for Slice {
    fn from(value: Txindex) -> Self {
        value.to_be_bytes().into()
    }
}
