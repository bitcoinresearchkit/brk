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
pub struct OpReturnIndex(TypeIndex);

impl From<TypeIndex> for OpReturnIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}

impl From<OpReturnIndex> for usize {
    #[inline]
    fn from(value: OpReturnIndex) -> Self {
        Self::from(*value)
    }
}

impl From<OpReturnIndex> for u64 {
    #[inline]
    fn from(value: OpReturnIndex) -> Self {
        Self::from(*value)
    }
}

impl From<usize> for OpReturnIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}

impl Add<usize> for OpReturnIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}

impl CheckedSub<OpReturnIndex> for OpReturnIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for OpReturnIndex {
    fn to_string() -> &'static str {
        "op_return_index"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["op", "opreturn", "op_return_index"]
    }
}

impl std::fmt::Display for OpReturnIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for OpReturnIndex {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.0.write_to(buf);
    }
}
