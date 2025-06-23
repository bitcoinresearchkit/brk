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
pub struct P2PK33AddressIndex(TypeIndex);
impl From<TypeIndex> for P2PK33AddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
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

impl Printable for P2PK33AddressIndex {
    fn to_string() -> &'static str {
        "p2pk33addressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["pk33addr", "p2pk33addr", "p2pk33addressindex"]
    }
}
