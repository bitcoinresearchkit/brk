use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use derive_deref::{Deref, DerefMut};
use iterator::bitcoin;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Height;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Amount(u64);

impl Amount {
    pub const ZERO: Self = Self(0);
    pub const ONE_BTC_F32: f32 = 100_000_000.0;
    pub const ONE_BTC_F64: f64 = 100_000_000.0;

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Amount) -> Self::Output {
        Amount::from(*self + *rhs)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, rhs: Amount) -> Self::Output {
        Amount::from(*self - *rhs)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Amount> for Amount {
    type Output = Amount;
    fn mul(self, rhs: Amount) -> Self::Output {
        Amount::from(*self * *rhs)
    }
}

impl Mul<u64> for Amount {
    type Output = Amount;
    fn mul(self, rhs: u64) -> Self::Output {
        Amount::from(*self * rhs)
    }
}

impl Mul<Height> for Amount {
    type Output = Amount;
    fn mul(self, rhs: Height) -> Self::Output {
        Amount::from(*self * *rhs as u64)
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sats: u64 = iter.map(|amt| *amt).sum();
        Amount::from(sats)
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<bitcoin::Amount> for Amount {
    fn from(value: bitcoin::Amount) -> Self {
        Self(value.to_sat())
    }
}
impl From<Amount> for bitcoin::Amount {
    fn from(value: Amount) -> Self {
        Self::from_sat(value.0)
    }
}
