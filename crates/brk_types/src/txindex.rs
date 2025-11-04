use std::{
    mem,
    ops::{Add, AddAssign},
};

use allocative::Allocative;
use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use redb::{TypeName, Value};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    Hash,
)]
pub struct TxIndex(u32);

impl TxIndex {
    pub const ZERO: Self = Self(0);
    pub const COINBASE: Self = Self(u32::MAX);

    pub fn new(txindex: u32) -> Self {
        Self(txindex)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    pub fn to_ne_bytes(&self) -> [u8; 4] {
        self.0.to_ne_bytes()
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
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<TxIndex> for u32 {
    #[inline]
    fn from(value: TxIndex) -> Self {
        value.0
    }
}

impl From<u64> for TxIndex {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for u64 {
    #[inline]
    fn from(value: TxIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for TxIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for usize {
    #[inline]
    fn from(value: TxIndex) -> Self {
        value.0 as usize
    }
}

impl From<ByteView> for TxIndex {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()))
    }
}
impl From<TxIndex> for ByteView {
    #[inline]
    fn from(value: TxIndex) -> Self {
        Self::new(&value.to_be_bytes())
    }
}

impl From<TxIndex> for StoredU32 {
    #[inline]
    fn from(value: TxIndex) -> Self {
        Self::from(value.0)
    }
}

impl From<&[u8]> for TxIndex {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(u32::from_be_bytes(copy_first_4bytes(value).unwrap()))
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

impl Value for TxIndex {
    type SelfType<'a> = TxIndex;
    type AsBytes<'a>
        = [u8; mem::size_of::<u32>()]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(mem::size_of::<u32>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> TxIndex
    where
        Self: 'a,
    {
        TxIndex(u32::from_le_bytes(data.try_into().unwrap()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; mem::size_of::<u32>()]
    where
        Self: 'a,
        Self: 'b,
    {
        value.0.to_le_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new("TxIndex")
    }
}
