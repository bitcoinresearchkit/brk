use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable, TypeIndex};

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
)]
pub struct P2SHAddressIndex(TypeIndex);
impl From<TypeIndex> for P2SHAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2SHAddressIndex> for TypeIndex {
    fn from(value: P2SHAddressIndex) -> Self {
        value.0
    }
}
impl From<P2SHAddressIndex> for u32 {
    fn from(value: P2SHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<u32> for P2SHAddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<P2SHAddressIndex> for usize {
    fn from(value: P2SHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2SHAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2SHAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2SHAddressIndex> for P2SHAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2SHAddressIndex {
    fn to_string() -> &'static str {
        "p2shaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["shaddr", "p2shaddr", "p2shaddressindex"]
    }
}
