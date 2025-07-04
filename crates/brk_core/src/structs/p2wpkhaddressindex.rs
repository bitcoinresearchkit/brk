use std::ops::Add;

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, IntoBytes};
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
impl From<P2WPKHAddressIndex> for TypeIndex {
    fn from(value: P2WPKHAddressIndex) -> Self {
        value.0
    }
}
impl From<P2WPKHAddressIndex> for usize {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<P2WPKHAddressIndex> for u32 {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<u32> for P2WPKHAddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
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

impl From<ByteView> for P2WPKHAddressIndex {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<P2WPKHAddressIndex> for ByteView {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(&value)
    }
}
impl From<&P2WPKHAddressIndex> for ByteView {
    fn from(value: &P2WPKHAddressIndex) -> Self {
        Self::new(value.as_bytes())
    }
}
