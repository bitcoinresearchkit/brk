use std::ops::Add;

use byteview::ByteView;
use serde::Serialize;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

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
pub struct TypeIndex(u32);

impl TypeIndex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn copy_then_increment(&mut self) -> Self {
        let i = *self;
        self.increment();
        i
    }
}

impl From<u32> for TypeIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<TypeIndex> for u32 {
    fn from(value: TypeIndex) -> Self {
        value.0
    }
}

impl From<u64> for TypeIndex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<TypeIndex> for u64 {
    fn from(value: TypeIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for TypeIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<TypeIndex> for usize {
    fn from(value: TypeIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for TypeIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl Add<TypeIndex> for TypeIndex {
    type Output = Self;
    fn add(self, rhs: TypeIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl From<ByteView> for TypeIndex {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<TypeIndex> for ByteView {
    fn from(value: TypeIndex) -> Self {
        Self::new(value.as_bytes())
    }
}

impl CheckedSub<TypeIndex> for TypeIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
