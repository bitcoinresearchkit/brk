use std::ops::{Add, AddAssign, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::{InputIndex, OutputIndex, TxIndex};

#[derive(
    Debug,
    Deref,
    Clone,
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
pub struct StoredU64(u64);

impl StoredU64 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u64) -> Self {
        Self(counter)
    }
}

impl From<u64> for StoredU64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredU64 {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl CheckedSub<StoredU64> for StoredU64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredU64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl Add for StoredU64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredU64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<f64> for StoredU64 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u64)
    }
}

impl From<StoredU64> for f64 {
    fn from(value: StoredU64) -> Self {
        value.0 as f64
    }
}

impl From<TxIndex> for StoredU64 {
    fn from(value: TxIndex) -> Self {
        Self(*value as u64)
    }
}

impl From<InputIndex> for StoredU64 {
    fn from(value: InputIndex) -> Self {
        Self(*value)
    }
}

impl From<OutputIndex> for StoredU64 {
    fn from(value: OutputIndex) -> Self {
        Self(*value)
    }
}

impl Printable for StoredU64 {
    fn to_string() -> &'static str {
        "u64"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["u64"]
    }
}
