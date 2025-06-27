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
pub struct P2PKHAddressIndex(TypeIndex);
impl From<TypeIndex> for P2PKHAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PKHAddressIndex> for usize {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<P2PKHAddressIndex> for u32 {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<u32> for P2PKHAddressIndex {
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<usize> for P2PKHAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2PKHAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PKHAddressIndex> for P2PKHAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2PKHAddressIndex {
    fn to_string() -> &'static str {
        "p2pkhaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["pkhaddr", "p2pkhaddr", "p2pkhaddressindex"]
    }
}

impl From<ByteView> for P2PKHAddressIndex {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}
impl From<P2PKHAddressIndex> for ByteView {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(&value)
    }
}
impl From<&P2PKHAddressIndex> for ByteView {
    fn from(value: &P2PKHAddressIndex) -> Self {
        Self::new(value.as_bytes())
    }
}
