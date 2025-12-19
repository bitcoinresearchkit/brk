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
pub struct P2WSHAddressIndex(TypeIndex);

impl From<TypeIndex> for P2WSHAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<P2WSHAddressIndex> for TypeIndex {
    #[inline]
    fn from(value: P2WSHAddressIndex) -> Self {
        value.0
    }
}

impl From<P2WSHAddressIndex> for u32 {
    #[inline]
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<P2WSHAddressIndex> for u64 {
    #[inline]
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for P2WSHAddressIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<P2WSHAddressIndex> for usize {
    #[inline]
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(*value)
    }
}

impl From<usize> for P2WSHAddressIndex {
    #[inline]
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

impl PrintableIndex for P2WSHAddressIndex {
    fn to_string() -> &'static str {
        "p2wshaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["wshaddr", "p2wshaddr", "p2wshaddressindex"]
    }
}

impl std::fmt::Display for P2WSHAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2WSHAddressIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
