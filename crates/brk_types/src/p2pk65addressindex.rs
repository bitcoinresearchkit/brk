use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
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
    Pco,
    JsonSchema,
)]
pub struct P2PK65AddressIndex(TypeIndex);

impl From<TypeIndex> for P2PK65AddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2PK65AddressIndex> for TypeIndex {
    #[inline]
    fn from(value: P2PK65AddressIndex) -> Self {
        value.0
    }
}

impl From<P2PK65AddressIndex> for u32 {
    #[inline]
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2PK65AddressIndex> for u64 {
    #[inline]
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2PK65AddressIndex> for usize {
    #[inline]
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2PK65AddressIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<usize> for P2PK65AddressIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl Add<usize> for P2PK65AddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

impl CheckedSub<P2PK65AddressIndex> for P2PK65AddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2PK65AddressIndex {
    fn to_string() -> &'static str {
        "p2pk65addressindex"
    }
    fn to_possible_strings() -> &'static [&'static str] {
        &["pk65addr", "p2pk65addr", "p2pk65addressindex"]
    }
}

impl std::fmt::Display for P2PK65AddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2PK65AddressIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
