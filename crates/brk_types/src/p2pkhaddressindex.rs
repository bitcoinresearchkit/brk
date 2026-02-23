use std::ops::Add;

use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

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
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct P2PKHAddressIndex(TypeIndex);

impl From<TypeIndex> for P2PKHAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2PKHAddressIndex> for TypeIndex {
    #[inline]
    fn from(value: P2PKHAddressIndex) -> Self {
        value.0
    }
}

impl From<P2PKHAddressIndex> for usize {
    #[inline]
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2PKHAddressIndex> for u64 {
    #[inline]
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2PKHAddressIndex> for u32 {
    #[inline]
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2PKHAddressIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<usize> for P2PKHAddressIndex {
    #[inline]
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

impl PrintableIndex for P2PKHAddressIndex {
    fn to_string() -> &'static str {
        "p2pkhaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["pkhaddr", "p2pkhaddr", "p2pkhaddressindex"]
    }
}

impl std::fmt::Display for P2PKHAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2PKHAddressIndex {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
