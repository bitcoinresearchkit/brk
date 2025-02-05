use std::ops::Add;

use derive_deref::{Deref, DerefMut};
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
pub struct Addresstypeindex(u32);

impl Addresstypeindex {
    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn clone_then_increment(&mut self) -> Self {
        let i = *self;
        self.increment();
        i
    }
}

impl From<u32> for Addresstypeindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Addresstypeindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Addresstypeindex> for u64 {
    fn from(value: Addresstypeindex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for Addresstypeindex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Addresstypeindex> for usize {
    fn from(value: Addresstypeindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Addresstypeindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<Addresstypeindex> for Addresstypeindex {
    type Output = Self;
    fn add(self, rhs: Addresstypeindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
