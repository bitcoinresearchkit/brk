use std::ops::{Add, AddAssign, Sub};

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Vin;

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
    Serialize,
)]
pub struct Txinindex(u64);

impl Txinindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<Txinindex> for Txinindex {
    type Output = Self;
    fn add(self, rhs: Txinindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vin> for Txinindex {
    type Output = Self;
    fn add(self, rhs: Vin) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl Add<usize> for Txinindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<Txinindex> for Txinindex {
    fn add_assign(&mut self, rhs: Txinindex) {
        self.0 += rhs.0
    }
}

impl Sub<Txinindex> for Txinindex {
    type Output = Self;
    fn sub(self, rhs: Txinindex) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Txinindex> for u32 {
    fn from(value: Txinindex) -> Self {
        if value.0 > u32::MAX as u64 {
            panic!()
        }
        value.0 as u32
    }
}

impl From<u64> for Txinindex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Txinindex> for u64 {
    fn from(value: Txinindex) -> Self {
        value.0
    }
}

impl From<usize> for Txinindex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<Txinindex> for usize {
    fn from(value: Txinindex) -> Self {
        value.0 as usize
    }
}
