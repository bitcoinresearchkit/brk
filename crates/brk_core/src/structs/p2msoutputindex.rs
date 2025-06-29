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
pub struct P2MSOutputIndex(TypeIndex);
impl From<TypeIndex> for P2MSOutputIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2MSOutputIndex> for usize {
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2MSOutputIndex {
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

impl Printable for P2MSOutputIndex {
    fn to_string() -> &'static str {
        "p2msoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["msout", "p2msout", "p2msoutputindex"]
    }
}
