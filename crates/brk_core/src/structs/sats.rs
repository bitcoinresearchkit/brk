use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, SubAssign},
};

use bitcoin::Amount;
use byteview::ByteView;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, copy_first_8bytes};

use super::{Bitcoin, Cents, Dollars, Height};

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

#[allow(clippy::inconsistent_digit_grouping)]
impl Sats {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(u64::MAX);
    pub const ONE_BTC: Self = Self(1_00_000_000);
    pub const FIFTY_BTC: Self = Self(50_00_000_000);

    pub fn new(sats: u64) -> Self {
        Self(sats)
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn is_not_zero(&self) -> bool {
        *self != Self::ZERO
    }
}

impl Add for Sats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
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

impl CheckedSub<usize> for Sats {
    fn checked_sub(self, rhs: usize) -> Option<Self> {
        self.0.checked_sub(rhs as u64).map(Self::from)
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
        Sats::from(self.0.checked_mul(rhs.0).unwrap())
    }
}

impl Mul<usize> for Sats {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Sats::from(self.0.checked_mul(rhs as u64).unwrap())
    }
}

impl Mul<u64> for Sats {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Sats::from(self.0.checked_mul(rhs).unwrap())
    }
}

impl Mul<Height> for Sats {
    type Output = Self;
    fn mul(self, rhs: Height) -> Self::Output {
        Sats::from(self.0.checked_mul(u64::from(rhs)).unwrap())
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
        let raw_cents = u64::from(Cents::from(rhs));
        if raw_cents != 0 {
            Self(self.0 * 100 / raw_cents)
        } else {
            Self::MAX
        }
    }
}

impl Div<Sats> for Sats {
    type Output = Self;
    fn div(self, rhs: Sats) -> Self::Output {
        if rhs.0 == 0 {
            Self(0)
        } else {
            Self(self.0 / rhs.0)
        }
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
        Self(value.round() as u64)
    }
}

impl From<Sats> for f64 {
    fn from(value: Sats) -> Self {
        value.0 as f64
    }
}

impl From<Sats> for usize {
    fn from(value: Sats) -> Self {
        value.0 as usize
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
        Self((f64::from(value) * (Sats::ONE_BTC.0 as f64)).round() as u64)
    }
}

impl From<Sats> for u64 {
    fn from(value: Sats) -> Self {
        value.0
    }
}

impl From<u128> for Sats {
    fn from(value: u128) -> Self {
        if value > u64::MAX as u128 {
            panic!("u128 bigger than u64")
        }
        Self(value as u64)
    }
}

impl From<Sats> for u128 {
    fn from(value: Sats) -> Self {
        value.0 as u128
    }
}

impl From<ByteView> for Sats {
    fn from(value: ByteView) -> Self {
        let bytes = copy_first_8bytes(&value).unwrap();
        Self::from(u64::from_be_bytes(bytes))
    }
}

impl From<&Sats> for ByteView {
    fn from(value: &Sats) -> Self {
        Self::new(&value.0.to_be_bytes())
    }
}

impl From<Sats> for ByteView {
    fn from(value: Sats) -> Self {
        Self::from(&value)
    }
}

impl Mul<Sats> for usize {
    type Output = Sats;
    fn mul(self, rhs: Sats) -> Self::Output {
        Self::Output::from(rhs.0 * self as u64)
    }
}
