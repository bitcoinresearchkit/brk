use std::ops::Add;

use brk_vecs::{CheckedSub, Printable, StoredCompressed};
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
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
pub struct UnknownOutputIndex(TypeIndex);

impl From<TypeIndex> for UnknownOutputIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<UnknownOutputIndex> for u64 {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<UnknownOutputIndex> for usize {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for UnknownOutputIndex {
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

impl Printable for UnknownOutputIndex {
    fn to_string() -> &'static str {
        "unknownoutputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["unknownout", "unknownoutputindex"]
    }
}
