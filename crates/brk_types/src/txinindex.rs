use std::ops::{Add, AddAssign};

use allocative::Allocative;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
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
    StoredCompressed,
    Allocative,
)]
pub struct TxInIndex(u64);

impl TxInIndex {
    pub fn new(index: u64) -> Self {
        Self(index)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<TxInIndex> for TxInIndex {
    type Output = Self;
    fn add(self, rhs: TxInIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vin> for TxInIndex {
    type Output = Self;
    fn add(self, rhs: Vin) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl Add<usize> for TxInIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<TxInIndex> for TxInIndex {
    fn add_assign(&mut self, rhs: TxInIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<TxInIndex> for TxInIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self::from)
    }
}

impl From<TxInIndex> for u32 {
    #[inline]
    fn from(value: TxInIndex) -> Self {
        if value.0 > u32::MAX as u64 {
            panic!()
        }
        value.0 as u32
    }
}

impl From<u64> for TxInIndex {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<TxInIndex> for u64 {
    #[inline]
    fn from(value: TxInIndex) -> Self {
        value.0
    }
}

impl From<usize> for TxInIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<TxInIndex> for usize {
    #[inline]
    fn from(value: TxInIndex) -> Self {
        value.0 as usize
    }
}

impl PrintableIndex for TxInIndex {
    fn to_string() -> &'static str {
        "txinindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["txi", "txin", "txinindex"]
    }
}

impl std::fmt::Display for TxInIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
