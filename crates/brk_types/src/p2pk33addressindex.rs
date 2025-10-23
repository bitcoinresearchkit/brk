use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    StoredCompressed,
)]
pub struct P2PK33AddressIndex(TypeIndex);
impl From<TypeIndex> for P2PK33AddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PK33AddressIndex> for TypeIndex {
    fn from(value: P2PK33AddressIndex) -> Self {
        value.0
    }
}
impl From<P2PK33AddressIndex> for u32 {
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<P2PK33AddressIndex> for u64 {
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<u32> for P2PK33AddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<P2PK33AddressIndex> for usize {
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PK33AddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2PK33AddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PK33AddressIndex> for P2PK33AddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2PK33AddressIndex {
    fn to_string() -> &'static str {
        "p2pk33addressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["pk33addr", "p2pk33addr", "p2pk33addressindex"]
    }
}

impl std::fmt::Display for P2PK33AddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
