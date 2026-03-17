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
pub struct FundedAddrIndex(TypeIndex);

impl From<TypeIndex> for FundedAddrIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<usize> for FundedAddrIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl From<FundedAddrIndex> for usize {
    #[inline]
    fn from(value: FundedAddrIndex) -> Self {
        usize::from(value.0)
    }
}
impl From<FundedAddrIndex> for u32 {
    #[inline]
    fn from(value: FundedAddrIndex) -> Self {
        u32::from(value.0)
    }
}
impl Add<usize> for FundedAddrIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl CheckedSub<FundedAddrIndex> for FundedAddrIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
impl PrintableIndex for FundedAddrIndex {
    fn to_string() -> &'static str {
        "funded_addr_index"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["fundedaddr", "funded_addr_index"]
    }
}

impl std::fmt::Display for FundedAddrIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for FundedAddrIndex {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.0.write_to(buf);
    }
}
