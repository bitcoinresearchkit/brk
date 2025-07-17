use std::ops::Add;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable, TypeIndex};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deref,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct EmptyAddressIndex(TypeIndex);

impl From<TypeIndex> for EmptyAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for EmptyAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<u32> for EmptyAddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<EmptyAddressIndex> for usize {
    fn from(value: EmptyAddressIndex) -> Self {
        usize::from(value.0)
    }
}

impl Add<usize> for EmptyAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl CheckedSub<EmptyAddressIndex> for EmptyAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl Printable for EmptyAddressIndex {
    fn to_string() -> &'static str {
        "emptyaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["emptyaddr", "emptyaddressindex"]
    }
}
