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
pub struct P2AAddressIndex(TypeIndex);
impl From<TypeIndex> for P2AAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2AAddressIndex> for usize {
    fn from(value: P2AAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2AAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2AAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2AAddressIndex> for P2AAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl Printable for P2AAddressIndex {
    fn to_string() -> &'static str {
        "p2aaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["aaddr", "p2aaddr", "p2aaddressindex"]
    }
}
