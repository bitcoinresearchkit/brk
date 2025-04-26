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
    PartialOrd,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct StoredF64(f64);

impl From<f64> for StoredF64 {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredF64 {
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl CheckedSub<StoredF64> for StoredF64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0 - rhs.0))
    }
}

impl Div<usize> for StoredF64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as f64)
    }
}

impl Add for StoredF64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<StoredF64> for f64 {
    fn from(value: StoredF64) -> Self {
        value.0
    }
}

impl Eq for StoredF64 {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for StoredF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
