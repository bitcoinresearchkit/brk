use std::ops::Add;

use derive_more::Deref;
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
    Default,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct FundedAddressIndex(TypeIndex);

impl From<TypeIndex> for FundedAddressIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for FundedAddressIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<FundedAddressIndex> for usize {
    #[inline]
    fn from(value: FundedAddressIndex) -> Self {
        usize::from(value.0)
    }
}
impl From<FundedAddressIndex> for u32 {
    #[inline]
    fn from(value: FundedAddressIndex) -> Self {
        u32::from(value.0)
    }
}
impl Add<usize> for FundedAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl CheckedSub<FundedAddressIndex> for FundedAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl PrintableIndex for FundedAddressIndex {
    fn to_string() -> &'static str {
        "fundedaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["fundedaddr", "fundedaddressindex"]
    }
}

impl std::fmt::Display for FundedAddressIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for FundedAddressIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
