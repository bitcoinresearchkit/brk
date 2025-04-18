use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Txindex, Txinindex, Txoutindex};

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

impl From<Txindex> for StoredU64 {
    fn from(value: Txindex) -> Self {
        Self(*value as u64)
    }
}

impl From<Txinindex> for StoredU64 {
    fn from(value: Txinindex) -> Self {
        Self(*value)
    }
}

impl From<Txoutindex> for StoredU64 {
    fn from(value: Txoutindex) -> Self {
        Self(*value)
    }
}
