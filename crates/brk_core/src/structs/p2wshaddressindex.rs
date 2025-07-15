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
pub struct P2WSHAddressIndex(TypeIndex);
impl From<TypeIndex> for P2WSHAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2WSHAddressIndex> for TypeIndex {
    fn from(value: P2WSHAddressIndex) -> Self {
        value.0
    }
}
impl From<P2WSHAddressIndex> for u32 {
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<u32> for P2WSHAddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<P2WSHAddressIndex> for usize {
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2WSHAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2WSHAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2WSHAddressIndex> for P2WSHAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2WSHAddressIndex {
    fn to_string() -> &'static str {
        "p2wshaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["wshaddr", "p2wshaddr", "p2wshaddressindex"]
    }
}

impl From<ByteView> for P2WSHAddressIndex {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<P2WSHAddressIndex> for ByteView {
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(&value)
    }
}
impl From<&P2WSHAddressIndex> for ByteView {
    fn from(value: &P2WSHAddressIndex) -> Self {
        Self::new(value.as_bytes())
    }
}
