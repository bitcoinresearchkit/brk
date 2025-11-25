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
pub struct EmptyOutputIndex(TypeIndex);
impl From<TypeIndex> for EmptyOutputIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<EmptyOutputIndex> for u64 {
    #[inline]
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(value.0)
    }
}
impl From<EmptyOutputIndex> for usize {
    #[inline]
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(value.0)
    }
}
impl From<usize> for EmptyOutputIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for EmptyOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl CheckedSub<EmptyOutputIndex> for EmptyOutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for EmptyOutputIndex {
    fn to_string() -> &'static str {
        "emptyoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["emptyout", "emptyoutputindex"]
    }
}

impl std::fmt::Display for EmptyOutputIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for EmptyOutputIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
