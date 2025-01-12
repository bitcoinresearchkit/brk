use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txoutindex(u64);

impl Txoutindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }
}

impl Add<Txoutindex> for Txoutindex {
    type Output = Self;
    fn add(self, rhs: Txoutindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Txoutindex> for Txoutindex {
    fn add_assign(&mut self, rhs: Txoutindex) {
        self.0 += rhs.0
    }
}

impl From<u64> for Txoutindex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Txoutindex> for u64 {
    fn from(value: Txoutindex) -> Self {
        value.0
    }
}

impl From<usize> for Txoutindex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<Txoutindex> for usize {
    fn from(value: Txoutindex) -> Self {
        value.0 as usize
    }
}

impl TryFrom<Slice> for Txoutindex {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::try_from(&value[..])?)
    }
}
impl TryFrom<&[u8]> for Txoutindex {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from(value.read_be_u64()?))
    }
}
impl From<Txoutindex> for Slice {
    fn from(value: Txoutindex) -> Self {
        value.to_be_bytes().into()
    }
}
