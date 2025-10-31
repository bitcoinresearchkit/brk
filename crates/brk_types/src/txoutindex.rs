use std::ops::{Add, AddAssign};

use allocative::Allocative;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_8bytes;

use super::Vout;

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
)]
pub struct TxOutIndex(u64);

impl TxOutIndex {
    pub const ZERO: Self = Self(0);

    pub const COINBASE: Self = Self(u64::MAX);

    pub fn new(index: u64) -> Self {
        Self(index)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }
}

impl Add<TxOutIndex> for TxOutIndex {
    type Output = Self;
    fn add(self, rhs: TxOutIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vout> for TxOutIndex {
    type Output = Self;
    fn add(self, rhs: Vout) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl Add<usize> for TxOutIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<TxOutIndex> for TxOutIndex {
    fn add_assign(&mut self, rhs: TxOutIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<TxOutIndex> for TxOutIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self::from)
    }
}

impl From<TxOutIndex> for u32 {
    #[inline]
    fn from(value: TxOutIndex) -> Self {
        if value.0 > u32::MAX as u64 {
            panic!()
        }
        value.0 as u32
    }
}

impl From<u64> for TxOutIndex {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<TxOutIndex> for u64 {
    #[inline]
    fn from(value: TxOutIndex) -> Self {
        value.0
    }
}

impl From<usize> for TxOutIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<TxOutIndex> for usize {
    #[inline]
    fn from(value: TxOutIndex) -> Self {
        value.0 as usize
    }
}

impl From<&[u8]> for TxOutIndex {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(u64::from_be_bytes(copy_first_8bytes(value).unwrap()))
    }
}

impl PrintableIndex for TxOutIndex {
    fn to_string() -> &'static str {
        "txoutindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["txo", "txout", "txoutindex"]
    }
}

impl std::fmt::Display for TxOutIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
