use core::panic;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::{Dollars, StoredF64};

#[derive(
    Debug, Deref, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize,
)]
pub struct StoredF32(f32);

impl From<f32> for StoredF32 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<f64> for StoredF32 {
    fn from(value: f64) -> Self {
        if value > f32::MAX as f64 {
            panic!("f64 is too big")
        }
        Self(value as f32)
    }
}

impl From<StoredF32> for f64 {
    fn from(value: StoredF32) -> Self {
        value.0 as f64
    }
}

impl From<StoredF64> for StoredF32 {
    fn from(value: StoredF64) -> Self {
        Self(*value as f32)
    }
}

impl From<usize> for StoredF32 {
    fn from(value: usize) -> Self {
        Self(value as f32)
    }
}

impl CheckedSub<StoredF32> for StoredF32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0 - rhs.0))
    }
}

impl Div<usize> for StoredF32 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as f32)
    }
}

impl Add for StoredF32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredF32 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<StoredF32> for f32 {
    fn from(value: StoredF32) -> Self {
        value.0
    }
}

impl From<Dollars> for StoredF32 {
    fn from(value: Dollars) -> Self {
        StoredF32::from(f64::from(value))
    }
}

impl Div<Dollars> for StoredF32 {
    type Output = Self;
    fn div(self, rhs: Dollars) -> Self::Output {
        Self::from(self.0 as f64 / *rhs)
    }
}

impl Div<StoredF32> for StoredF32 {
    type Output = Self;
    fn div(self, rhs: StoredF32) -> Self::Output {
        Self::from(self.0 / rhs.0)
    }
}

impl Mul<usize> for StoredF32 {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as f32)
    }
}

impl Mul<StoredF32> for usize {
    type Output = StoredF32;
    fn mul(self, rhs: StoredF32) -> Self::Output {
        StoredF32(self as f32 * rhs.0)
    }
}

impl Sub<StoredF32> for StoredF32 {
    type Output = Self;
    fn sub(self, rhs: StoredF32) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl PartialEq for StoredF32 {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for StoredF32 {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for StoredF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for StoredF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}

impl Printable for StoredF32 {
    fn to_string() -> &'static str {
        "f32"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["f32"]
    }
}
