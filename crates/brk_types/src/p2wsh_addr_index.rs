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
pub struct P2WSHAddrIndex(TypeIndex);

impl From<TypeIndex> for P2WSHAddrIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2WSHAddrIndex> for TypeIndex {
    #[inline]
    fn from(value: P2WSHAddrIndex) -> Self {
        value.0
    }
}

impl From<P2WSHAddrIndex> for u32 {
    #[inline]
    fn from(value: P2WSHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2WSHAddrIndex> for u64 {
    #[inline]
    fn from(value: P2WSHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2WSHAddrIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<P2WSHAddrIndex> for usize {
    #[inline]
    fn from(value: P2WSHAddrIndex) -> Self {
        Self::from(*value)
    }
}

impl From<usize> for P2WSHAddrIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl Add<usize> for P2WSHAddrIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

impl CheckedSub<P2WSHAddrIndex> for P2WSHAddrIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2WSHAddrIndex {
    fn to_string() -> &'static str {
        "p2wsh_addr_index"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["wshaddr", "p2wshaddr", "p2wsh_addr_index"]
    }
}

impl std::fmt::Display for P2WSHAddrIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2WSHAddrIndex {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.0.write_to(buf);
    }
}
