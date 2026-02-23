use std::ops::Add;

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use crate::TypeIndex;

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
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct EmptyAddressIndex(TypeIndex);

impl From<TypeIndex> for EmptyAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for EmptyAddressIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<u32> for EmptyAddressIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl From<EmptyAddressIndex> for usize {
    #[inline]
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

impl PrintableIndex for EmptyAddressIndex {
    fn to_string() -> &'static str {
        "emptyaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["emptyaddr", "emptyaddressindex"]
    }
}

impl std::fmt::Display for EmptyAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for EmptyAddressIndex {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
