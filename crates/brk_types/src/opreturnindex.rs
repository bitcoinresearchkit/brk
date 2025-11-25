use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    Pco,
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
        "opreturnindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["op", "opreturn", "opreturnindex"]
    }
}

impl std::fmt::Display for OpReturnIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for OpReturnIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
