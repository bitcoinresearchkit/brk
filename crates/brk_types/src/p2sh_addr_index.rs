use std::ops::Add;

use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use crate::TypeIndex;

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
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct P2SHAddrIndex(TypeIndex);

impl From<TypeIndex> for P2SHAddrIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2SHAddrIndex> for TypeIndex {
    #[inline]
    fn from(value: P2SHAddrIndex) -> Self {
        value.0
    }
}

impl From<P2SHAddrIndex> for u32 {
    #[inline]
    fn from(value: P2SHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2SHAddrIndex> for u64 {
    #[inline]
    fn from(value: P2SHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2SHAddrIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<u64> for P2SHAddrIndex {
    #[inline]
    fn from(value: u64) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<P2SHAddrIndex> for usize {
    #[inline]
    fn from(value: P2SHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<usize> for P2SHAddrIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl Add<usize> for P2SHAddrIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

impl CheckedSub<P2SHAddrIndex> for P2SHAddrIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2SHAddrIndex {
    fn to_string() -> &'static str {
        "p2sh_addr_index"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["shaddr", "p2shaddr", "p2sh_addr_index"]
    }
}

impl std::fmt::Display for P2SHAddrIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2SHAddrIndex {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.0.write_to(buf);
    }
}
