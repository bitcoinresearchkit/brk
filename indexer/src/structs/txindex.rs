use std::ops::{Add, AddAssign, Sub};

use derive_deref::{Deref, DerefMut};
use fjall::Slice;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
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

impl Add<usize> for Txindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl AddAssign<Txindex> for Txindex {
    fn add_assign(&mut self, rhs: Txindex) {
        self.0 += rhs.0
    }
}

impl Sub<Txindex> for Txindex {
    type Output = Txindex;
    fn sub(self, rhs: Txindex) -> Self::Output {
        Self::from(*self - *rhs)
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
    type Error = storable_vec::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<Txindex> for Slice {
    fn from(value: Txindex) -> Self {
        Self::new(value.as_bytes())
    }
}
