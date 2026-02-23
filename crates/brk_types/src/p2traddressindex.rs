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
pub struct P2TRAddressIndex(TypeIndex);

impl From<TypeIndex> for P2TRAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2TRAddressIndex> for TypeIndex {
    #[inline]
    fn from(value: P2TRAddressIndex) -> Self {
        value.0
    }
}

impl From<P2TRAddressIndex> for u32 {
    #[inline]
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2TRAddressIndex> for u64 {
    #[inline]
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2TRAddressIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<P2TRAddressIndex> for usize {
    #[inline]
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<usize> for P2TRAddressIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl Add<usize> for P2TRAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

impl CheckedSub<P2TRAddressIndex> for P2TRAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2TRAddressIndex {
    fn to_string() -> &'static str {
        "p2traddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["traddr", "p2traddr", "p2traddressindex"]
    }
}

impl std::fmt::Display for P2TRAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2TRAddressIndex {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
