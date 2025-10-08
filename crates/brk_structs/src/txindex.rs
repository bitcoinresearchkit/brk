use std::ops::{Add, AddAssign};

use allocative::Allocative;
use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_4bytes;

use super::StoredU32;

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
    StoredCompressed,
    Allocative,
    JsonSchema,
)]
pub struct TxIndex(u32);

impl TxIndex {
    pub const ZERO: Self = Self(0);

    pub fn new(txindex: u32) -> Self {
        Self(txindex)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<TxIndex> for TxIndex {
    type Output = Self;
    fn add(self, rhs: TxIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<usize> for TxIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl AddAssign<TxIndex> for TxIndex {
    fn add_assign(&mut self, rhs: TxIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<TxIndex> for TxIndex {
    fn checked_sub(self, rhs: TxIndex) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(TxIndex::from)
    }
}

impl From<u32> for TxIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for TxIndex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for u64 {
    fn from(value: TxIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for TxIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for usize {
    fn from(value: TxIndex) -> Self {
        value.0 as usize
    }
}

impl From<ByteView> for TxIndex {
    fn from(value: ByteView) -> Self {
        Self(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()))
    }
}
impl From<TxIndex> for ByteView {
    fn from(value: TxIndex) -> Self {
        Self::new(&value.to_be_bytes())
    }
}

impl From<TxIndex> for StoredU32 {
    fn from(value: TxIndex) -> Self {
        Self::from(value.0)
    }
}

impl PrintableIndex for TxIndex {
    fn to_string() -> &'static str {
        "txindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["tx", "txindex"]
    }
}

impl std::fmt::Display for TxIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
