use std::ops::{Add, AddAssign};

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Error};

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
pub struct Txindex(u32);

impl Txindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
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

impl CheckedSub<Txindex> for Txindex {
    fn checked_sub(self, rhs: Txindex) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Txindex::from)
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

impl TryFrom<ByteView> for Txindex {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<Txindex> for ByteView {
    fn from(value: Txindex) -> Self {
        Self::new(value.as_bytes())
    }
}
