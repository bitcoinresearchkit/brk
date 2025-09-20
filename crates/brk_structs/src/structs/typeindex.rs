use std::ops::Add;

use byteview::ByteView;
use serde::Serialize;
use vecdb::{CheckedSub, StoredCompressed};
use zerocopy::IntoBytes;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_4bytes;

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
    StoredCompressed,
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

impl Add<u32> for TypeIndex {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
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

impl From<&[u8]> for TypeIndex {
    fn from(value: &[u8]) -> Self {
        Self(u32::from_be_bytes(copy_first_4bytes(value).unwrap()))
    }
}

impl From<ByteView> for TypeIndex {
    fn from(value: ByteView) -> Self {
        Self::from(value.as_bytes())
    }
}
impl From<TypeIndex> for ByteView {
    fn from(value: TypeIndex) -> Self {
        Self::new(&value.0.to_be_bytes())
    }
}

impl CheckedSub<TypeIndex> for TypeIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl std::fmt::Display for TypeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
