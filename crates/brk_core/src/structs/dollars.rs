use std::{
    f64,
    ops::{Add, AddAssign, Div, Mul},
};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Bitcoin, Cents, Close, Sats, StoredF32, StoredF64};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Deref,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Dollars(f64);

impl Dollars {
    pub const ZERO: Self = Self(0.0);

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
        StoredF64::from(self.0 / rhs.0)
    }
}

impl Div<Close<Dollars>> for Dollars {
    type Output = StoredF64;
    fn div(self, rhs: Close<Dollars>) -> Self::Output {
        StoredF64::from(self.0 / rhs.0)
    }
}

impl Div<Dollars> for Close<Dollars> {
    type Output = StoredF64;
    fn div(self, rhs: Dollars) -> Self::Output {
        StoredF64::from(self.0 / rhs.0)
    }
}

impl Div<usize> for Dollars {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(Cents::from(self) / rhs)
    }
}

impl Div<Bitcoin> for Dollars {
    type Output = Self;
    fn div(self, rhs: Bitcoin) -> Self::Output {
        Self(f64::from(self) / f64::from(rhs))
    }
}

impl Eq for Dollars {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Dollars {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Mul<Bitcoin> for Dollars {
    type Output = Self;
    fn mul(self, rhs: Bitcoin) -> Self::Output {
        Self::from(Cents::from(
            u128::from(Sats::from(rhs)) * u128::from(Cents::from(self)) / u128::from(Sats::ONE_BTC),
        ))
    }
}

impl Mul<StoredF32> for Dollars {
    type Output = Self;
    fn mul(self, rhs: StoredF32) -> Self::Output {
        if rhs.is_nan() {
            Self(f64::NAN)
        } else {
            Self::from(Cents::from(Self::from(self.0 * *rhs as f64)))
        }
    }
}

impl Mul<usize> for Dollars {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self::from(Cents::from(self) * rhs)
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
        Cents::from(self)
            .checked_sub(Cents::from(rhs))
            .map(Dollars::from)
    }
}
