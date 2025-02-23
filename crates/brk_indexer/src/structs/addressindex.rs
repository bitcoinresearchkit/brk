use std::ops::Add;

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
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
    Serialize,
)]
pub struct Addressindex(u32);

impl Addressindex {
    pub const BYTES: usize = size_of::<Self>();

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl From<u32> for Addressindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Addressindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Addressindex> for u64 {
    fn from(value: Addressindex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for Addressindex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Addressindex> for usize {
    fn from(value: Addressindex) -> Self {
        value.0 as usize
    }
}

impl TryFrom<ByteView> for Addressindex {
    type Error = storable_vec::Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<Addressindex> for ByteView {
    fn from(value: Addressindex) -> Self {
        Self::new(value.as_bytes())
    }
}

impl Add<usize> for Addressindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<Addressindex> for Addressindex {
    type Output = Self;
    fn add(self, rhs: Addressindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
