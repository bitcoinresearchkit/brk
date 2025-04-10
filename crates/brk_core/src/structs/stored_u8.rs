use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

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
pub struct StoredU8(u8);

impl StoredU8 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u8) -> Self {
        Self(counter)
    }
}

impl From<u8> for StoredU8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredU8 {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl CheckedSub<StoredU8> for StoredU8 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredU8 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u8)
    }
}

impl Add for StoredU8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for StoredU8 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u8)
    }
}

impl From<StoredU8> for f64 {
    fn from(value: StoredU8) -> Self {
        value.0 as f64
    }
}
