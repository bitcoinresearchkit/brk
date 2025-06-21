use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::{
    EmptyOutputIndex, OpReturnIndex, P2AIndex, P2MSIndex, P2PK33Index, P2PK65Index, P2PKHIndex,
    P2SHIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, UnknownOutputIndex,
};

#[derive(
    Debug,
    Deref,
    Clone,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct StoredU32(u32);

impl StoredU32 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u32) -> Self {
        Self(counter)
    }
}

impl From<u32> for StoredU32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredU32 {
    fn from(value: usize) -> Self {
        if value > u32::MAX as usize {
            panic!("usize too big (value = {value})")
        }
        Self(value as u32)
    }
}

impl CheckedSub<StoredU32> for StoredU32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredU32 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u32)
    }
}

impl Add for StoredU32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for StoredU32 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u32)
    }
}

impl From<StoredU32> for f64 {
    fn from(value: StoredU32) -> Self {
        value.0 as f64
    }
}

impl From<StoredU32> for usize {
    fn from(value: StoredU32) -> Self {
        value.0 as usize
    }
}

impl From<P2PK65Index> for StoredU32 {
    fn from(value: P2PK65Index) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK33Index> for StoredU32 {
    fn from(value: P2PK33Index) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PKHIndex> for StoredU32 {
    fn from(value: P2PKHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OpReturnIndex> for StoredU32 {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2MSIndex> for StoredU32 {
    fn from(value: P2MSIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2SHIndex> for StoredU32 {
    fn from(value: P2SHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WSHIndex> for StoredU32 {
    fn from(value: P2WSHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WPKHIndex> for StoredU32 {
    fn from(value: P2WPKHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2TRIndex> for StoredU32 {
    fn from(value: P2TRIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2AIndex> for StoredU32 {
    fn from(value: P2AIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<UnknownOutputIndex> for StoredU32 {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<EmptyOutputIndex> for StoredU32 {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl Printable for StoredU32 {
    fn to_string() -> &'static str {
        "u32"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["u32"]
    }
}
