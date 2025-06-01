use std::ops::Add;

use byteview::ByteView;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct AddressIndex(u32);

impl AddressIndex {
    pub const BYTES: usize = size_of::<Self>();

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u32> for AddressIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for AddressIndex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<AddressIndex> for u64 {
    fn from(value: AddressIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for AddressIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<AddressIndex> for usize {
    fn from(value: AddressIndex) -> Self {
        value.0 as usize
    }
}

impl From<ByteView> for AddressIndex {
    fn from(value: ByteView) -> Self {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<AddressIndex> for ByteView {
    fn from(value: AddressIndex) -> Self {
        Self::new(value.as_bytes())
    }
}

impl Add<usize> for AddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<AddressIndex> for AddressIndex {
    type Output = Self;
    fn add(self, rhs: AddressIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
