use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use biter::bitcoin;
use derive_deref::{Deref, DerefMut};

use super::Height;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Amount(bitcoin::Amount);

impl Amount {
    pub const ZERO: Self = Self(bitcoin::Amount::ZERO);
    pub const ONE_BTC_F32: f32 = 100_000_000.0;
    pub const ONE_BTC_F64: f64 = 100_000_000.0;
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(bitcoin::Amount::from_sat(value))
    }
}

impl From<bitcoin::Amount> for Amount {
    fn from(value: bitcoin::Amount) -> Self {
        Self(value)
    }
}

impl From<Amount> for fjall::Slice {
    fn from(value: Amount) -> Self {
        value.to_sat().to_be_bytes().into()
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Amount) -> Self::Output {
        Amount::from(self.to_sat() + rhs.to_sat())
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        *self = Amount::from(self.to_sat() + rhs.to_sat());
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, rhs: Amount) -> Self::Output {
        Amount::from(self.to_sat() - rhs.to_sat())
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Amount::from(self.to_sat() - rhs.to_sat());
    }
}

impl Mul<Amount> for Amount {
    type Output = Amount;
    fn mul(self, rhs: Amount) -> Self::Output {
        Amount::from(self.to_sat() * rhs.to_sat())
    }
}

impl Mul<u64> for Amount {
    type Output = Amount;
    fn mul(self, rhs: u64) -> Self::Output {
        Amount::from(self.to_sat() * rhs)
    }
}

impl Mul<Height> for Amount {
    type Output = Amount;
    fn mul(self, rhs: Height) -> Self::Output {
        Amount::from(self.to_sat() * *rhs as u64)
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sats: u64 = iter.map(|amt| amt.to_sat()).sum();
        Amount::from(sats)
    }
}
