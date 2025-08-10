use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use vecdb::{CheckedSub, Printable, StoredCompressed};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    StoredCompressed,
)]
pub struct EmptyOutputIndex(TypeIndex);
impl From<TypeIndex> for EmptyOutputIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<EmptyOutputIndex> for u64 {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(value.0)
    }
}
impl From<EmptyOutputIndex> for usize {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(value.0)
    }
}
impl From<usize> for EmptyOutputIndex {
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
impl Printable for EmptyOutputIndex {
    fn to_string() -> &'static str {
        "emptyoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["emptyout", "emptyoutputindex"]
    }
}
