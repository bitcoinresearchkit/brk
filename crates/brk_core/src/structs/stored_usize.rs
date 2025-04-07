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
pub struct StoredUsize(usize);

impl StoredUsize {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: usize) -> Self {
        Self(counter)
    }
}

impl From<usize> for StoredUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl CheckedSub<StoredUsize> for StoredUsize {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredUsize {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Add for StoredUsize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for StoredUsize {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as usize)
    }
}

impl From<StoredUsize> for f64 {
    fn from(value: StoredUsize) -> Self {
        value.0 as f64
    }
}
