use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use snkrj::{direct_repr, Storable, UnsizedStorable};
use storable_vec::UnsafeSizedSerDe;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txindex(u32);
direct_repr!(Txindex);

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

impl TryFrom<fjall::Slice> for Txindex {
    type Error = storable_vec::Error;
    fn try_from(value: fjall::Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<Txindex> for fjall::Slice {
    fn from(value: Txindex) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
