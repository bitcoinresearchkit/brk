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
pub struct P2MSOutputIndex(TypeIndex);
impl From<TypeIndex> for P2MSOutputIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2MSOutputIndex> for usize {
    #[inline]
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<P2MSOutputIndex> for u64 {
    #[inline]
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2MSOutputIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2MSOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2MSOutputIndex> for P2MSOutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for P2MSOutputIndex {
    fn to_string() -> &'static str {
        "p2msoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["msout", "p2msout", "p2msoutputindex"]
    }
}

impl std::fmt::Display for P2MSOutputIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for P2MSOutputIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
