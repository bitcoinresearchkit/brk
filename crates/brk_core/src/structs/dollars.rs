use std::{
    cmp::Ordering,
    f64,
    ops::{Add, AddAssign, Div, Mul},
};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Bitcoin, Cents, Close, Sats, StoredF32, StoredF64};

#[derive(
    Debug, Default, Clone, Copy, Deref, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize,
)]
pub struct Dollars(f64);

impl Dollars {
    pub const ZERO: Self = Self(0.0);
    pub const NAN: Self = Self(f64::NAN);

    pub const fn mint(dollars: f64) -> Self {
        Self(dollars)
    }
}

impl From<f32> for Dollars {
    fn from(value: f32) -> Self {
        Self(value as f64)
    }
}

impl From<f64> for Dollars {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Cents> for Dollars {
    fn from(value: Cents) -> Self {
        Self(f64::from(value) / 100.0)
    }
}

impl From<Dollars> for f32 {
    fn from(value: Dollars) -> Self {
        value.0 as f32
    }
}

impl From<Dollars> for f64 {
    fn from(value: Dollars) -> Self {
        value.0
    }
}

impl From<Close<Dollars>> for Dollars {
    fn from(value: Close<Dollars>) -> Self {
        Self(value.0)
    }
}

impl From<usize> for Dollars {
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl Add for Dollars {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(Cents::from(self) + Cents::from(rhs))
    }
}

impl Div<Dollars> for Dollars {
    type Output = StoredF64;
    fn div(self, rhs: Dollars) -> Self::Output {
        if self.is_nan() {
            StoredF64::from(self.0)
        } else {
            StoredF64::from(self.0 / rhs.0)
        }
    }
}

impl Div<Close<Dollars>> for Dollars {
    type Output = StoredF64;
    fn div(self, rhs: Close<Dollars>) -> Self::Output {
        if self.is_nan() {
            StoredF64::from(self.0)
        } else {
            StoredF64::from(self.0 / rhs.0)
        }
    }
}

impl Div<Dollars> for Close<Dollars> {
    type Output = StoredF64;
    fn div(self, rhs: Dollars) -> Self::Output {
        if self.is_nan() {
            StoredF64::from(self.0)
        } else {
            StoredF64::from(self.0 / rhs.0)
        }
    }
}

impl Div<usize> for Dollars {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        if self.is_nan() {
            self
        } else {
            Self::from(Cents::from(self) / rhs)
        }
    }
}

impl Div<Bitcoin> for Dollars {
    type Output = Self;
    fn div(self, rhs: Bitcoin) -> Self::Output {
        if self.is_nan() {
            self
        } else {
            Self(f64::from(self) / f64::from(rhs))
        }
    }
}

impl Mul<Bitcoin> for Dollars {
    type Output = Self;
    fn mul(self, rhs: Bitcoin) -> Self::Output {
        if self.is_nan() {
            self
        } else {
            Self::from(Cents::from(
                u128::from(Sats::from(rhs)) * u128::from(Cents::from(self))
                    / u128::from(Sats::ONE_BTC),
            ))
        }
    }
}

impl Mul<StoredF32> for Dollars {
    type Output = Self;
    fn mul(self, rhs: StoredF32) -> Self::Output {
        self * *rhs as f64
    }
}

impl Mul<f64> for Dollars {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        if rhs.is_nan() {
            self
        } else {
            Self::from(Cents::from(self) * Cents::from(Dollars::from(rhs)))
        }
    }
}

impl Mul<usize> for Dollars {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        if self.is_nan() {
            self
        } else {
            Self::from(Cents::from(self) * rhs)
        }
    }
}

impl From<u128> for Dollars {
    fn from(value: u128) -> Self {
        Self::from(Cents::from(value))
    }
}

impl From<Close<Dollars>> for u128 {
    fn from(value: Close<Dollars>) -> Self {
        u128::from(*value)
    }
}

impl From<Dollars> for u128 {
    fn from(value: Dollars) -> Self {
        u128::from(Cents::from(value))
    }
}

impl AddAssign for Dollars {
    fn add_assign(&mut self, rhs: Self) {
        *self = Dollars::from(Cents::from(*self) + Cents::from(rhs));
    }
}

impl CheckedSub for Dollars {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if self.is_nan() {
            Some(self)
        } else {
            Cents::from(self)
                .checked_sub(Cents::from(rhs))
                .map(Dollars::from)
        }
    }
}

impl CheckedSub<usize> for Dollars {
    fn checked_sub(self, rhs: usize) -> Option<Self> {
        Some(Dollars::from(
            Cents::from(self).checked_sub(Cents::from(rhs)).unwrap(),
        ))
    }
}

impl PartialEq for Dollars {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for Dollars {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for Dollars {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Dollars {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}
