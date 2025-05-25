use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul},
};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

#[derive(
    Debug, Deref, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize,
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

impl Mul<usize> for StoredF64 {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as f64)
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

impl CheckedSub<usize> for StoredF64 {
    fn checked_sub(self, rhs: usize) -> Option<Self> {
        Some(Self(self.0 - rhs as f64))
    }
}

impl PartialEq for StoredF64 {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for StoredF64 {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for StoredF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for StoredF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}
