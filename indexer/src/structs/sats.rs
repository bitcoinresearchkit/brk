use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use derive_deref::{Deref, DerefMut};
use iterator::bitcoin::Amount;
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
pub struct Sats(u64);

impl Sats {
    pub const ZERO: Self = Self(0);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Add for Sats {
    type Output = Sats;
    fn add(self, rhs: Sats) -> Self::Output {
        Sats::from(*self + *rhs)
    }
}

impl AddAssign for Sats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Sats {
    type Output = Sats;
    fn sub(self, rhs: Sats) -> Self::Output {
        Sats::from(*self - *rhs)
    }
}

impl SubAssign for Sats {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Sats> for Sats {
    type Output = Sats;
    fn mul(self, rhs: Sats) -> Self::Output {
        Sats::from(*self * *rhs)
    }
}

impl Mul<u64> for Sats {
    type Output = Sats;
    fn mul(self, rhs: u64) -> Self::Output {
        Sats::from(*self * rhs)
    }
}

impl Mul<Height> for Sats {
    type Output = Sats;
    fn mul(self, rhs: Height) -> Self::Output {
        Sats::from(*self * *rhs as u64)
    }
}

impl Sum for Sats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sats: u64 = iter.map(|sats| *sats).sum();
        Sats::from(sats)
    }
}

impl From<u64> for Sats {
    fn from(value: u64) -> Self {
        Self(value)
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
