use std::ops::Add;

use brk_vecs::{CheckedSub, Printable};
use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct LoadedAddressIndex(TypeIndex);

impl From<TypeIndex> for LoadedAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for LoadedAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<LoadedAddressIndex> for usize {
    fn from(value: LoadedAddressIndex) -> Self {
        usize::from(value.0)
    }
}
impl From<LoadedAddressIndex> for u32 {
    fn from(value: LoadedAddressIndex) -> Self {
        u32::from(value.0)
    }
}
impl Add<usize> for LoadedAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl CheckedSub<LoadedAddressIndex> for LoadedAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl Printable for LoadedAddressIndex {
    fn to_string() -> &'static str {
        "loadedaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["loadedaddr", "loadedaddressindex"]
    }
}
