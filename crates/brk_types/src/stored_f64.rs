use std::{
    cmp::Ordering,
    f64,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

use derive_deref::Deref;
use serde::Serialize;
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Bitcoin, Dollars};

#[derive(
    Debug, Deref, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize, Pco,
)]
pub struct StoredF64(f64);

impl StoredF64 {
    pub const NAN: Self = Self(f64::NAN);
}

impl From<f64> for StoredF64 {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<f32> for StoredF64 {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value as f64)
    }
}

impl From<u8> for StoredF64 {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value as f64)
    }
}

impl From<usize> for StoredF64 {
    #[inline]
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

impl Sub for StoredF64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for StoredF64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Dollars> for StoredF64 {
    type Output = Self;
    fn mul(self, rhs: Dollars) -> Self::Output {
        Self(self.0 * *rhs)
    }
}

impl Div<usize> for StoredF64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as f64)
    }
}

impl Div for StoredF64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<Dollars> for StoredF64 {
    type Output = Self;
    fn div(self, rhs: Dollars) -> Self::Output {
        Self::from(self.0 / *rhs)
    }
}

impl Add for StoredF64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredF64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<StoredF64> for f64 {
    #[inline]
    fn from(value: StoredF64) -> Self {
        value.0
    }
}

impl From<StoredF64> for f32 {
    #[inline]
    fn from(value: StoredF64) -> Self {
        value.0 as f32
    }
}

impl From<Dollars> for StoredF64 {
    #[inline]
    fn from(value: Dollars) -> Self {
        Self(f64::from(value))
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

impl From<Bitcoin> for StoredF64 {
    #[inline]
    fn from(value: Bitcoin) -> Self {
        Self(f64::from(value))
    }
}

impl PrintableIndex for StoredF64 {
    fn to_string() -> &'static str {
        "f64"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["f64"]
    }
}

impl Sum for StoredF64 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|v| v.0).sum::<f64>())
    }
}

impl Div<Bitcoin> for StoredF64 {
    type Output = Self;
    fn div(self, rhs: Bitcoin) -> Self::Output {
        Self(self.0 / f64::from(rhs))
    }
}

impl std::fmt::Display for StoredF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = ryu::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for StoredF64 {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
