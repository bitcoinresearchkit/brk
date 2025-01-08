use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txindex(u32);

impl Txindex {
    pub const BYTES: usize = size_of::<Self>();

    pub fn incremented(self) -> Self {
        Self(*self + 1)
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

impl From<Slice> for Txindex {
    fn from(slice: Slice) -> Self {
        Self(slice.read_u32())
    }
}
impl From<Txindex> for Slice {
    fn from(value: Txindex) -> Self {
        value.to_be_bytes().into()
    }
}

impl Add<usize> for Txindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl AddAssign<usize> for Txindex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs as u32
    }
}
