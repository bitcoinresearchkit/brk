use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, Compressable, Formattable, PrintableIndex};
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
    Compressable,
)]
pub struct UnknownOutputIndex(TypeIndex);

impl From<TypeIndex> for UnknownOutputIndex {
    #[inline]
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<UnknownOutputIndex> for u64 {
    #[inline]
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<UnknownOutputIndex> for usize {
    #[inline]
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for UnknownOutputIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for UnknownOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<UnknownOutputIndex> for UnknownOutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for UnknownOutputIndex {
    fn to_string() -> &'static str {
        "unknownoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["unknownout", "unknownoutputindex"]
    }
}

impl std::fmt::Display for UnknownOutputIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Formattable for UnknownOutputIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
