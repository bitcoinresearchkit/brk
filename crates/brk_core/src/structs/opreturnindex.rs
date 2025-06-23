use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable, TypeIndex};

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
)]
pub struct OpReturnIndex(TypeIndex);
impl From<TypeIndex> for OpReturnIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<OpReturnIndex> for usize {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for OpReturnIndex {
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

impl Printable for OpReturnIndex {
    fn to_string() -> &'static str {
        "opreturnindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["op", "opreturn", "opreturnindex"]
    }
}
