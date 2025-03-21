use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, SubAssign},
};

use bitcoin::Amount;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Bitcoin, Dollars, Height};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Sats(u64);

impl Sats {
    pub const ZERO: Self = Self(0);
    pub const ONE_BTC: Self = Self(100_000_000);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Add for Sats {
    type Output = Self;
    fn add(self, rhs: Sats) -> Self::Output {
        Sats::from(self.0 + rhs.0)
    }
}

impl AddAssign for Sats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl CheckedSub<Sats> for Sats {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self::from)
    }
}

impl SubAssign for Sats {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.checked_sub(rhs).unwrap();
    }
}

impl Mul<Sats> for Sats {
    type Output = Self;
    fn mul(self, rhs: Sats) -> Self::Output {
        Sats::from(self.0 * rhs.0)
    }
}

impl Mul<u64> for Sats {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Sats::from(self.0 * rhs)
    }
}

impl Mul<Height> for Sats {
    type Output = Self;
    fn mul(self, rhs: Height) -> Self::Output {
        Sats::from(self.0 * u64::from(rhs))
    }
}

impl Sum for Sats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sats: u64 = iter.map(|sats| sats.0).sum();
        Sats::from(sats)
    }
}

impl Div<Dollars> for Sats {
    type Output = Self;
    fn div(self, rhs: Dollars) -> Self::Output {
        Self((self.0 as f64 / f64::from(rhs)) as u64)
    }
}

impl Div<usize> for Sats {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl From<u64> for Sats {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for Sats {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<f64> for Sats {
    fn from(value: f64) -> Self {
        Self(value as u64)
    }
}

impl From<Sats> for f64 {
    fn from(value: Sats) -> Self {
        value.0 as f64
    }
}

impl From<Amount> for Sats {
    fn from(value: Amount) -> Self {
        Self(value.to_sat())
    }
}
impl From<Sats> for Amount {
    fn from(value: Sats) -> Self {
        Self::from_sat(value.0)
    }
}

impl From<Bitcoin> for Sats {
    fn from(value: Bitcoin) -> Self {
        Self((f64::from(value) * (u64::from(Sats::ONE_BTC) as f64)) as u64)
    }
}

impl From<Sats> for u64 {
    fn from(value: Sats) -> Self {
        value.0
    }
}
