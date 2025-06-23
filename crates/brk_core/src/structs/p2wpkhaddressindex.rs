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
pub struct P2WPKHAddressIndex(TypeIndex);
impl From<TypeIndex> for P2WPKHAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2WPKHAddressIndex> for usize {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WPKHAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2WPKHAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2WPKHAddressIndex> for P2WPKHAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2WPKHAddressIndex {
    fn to_string() -> &'static str {
        "p2wpkhaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["wpkhaddr", "p2wpkhaddr", "p2wpkhaddressindex"]
    }
}
