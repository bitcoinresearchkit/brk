use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::Amount;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    Allocative,
)]
pub struct Price(u64);

const SIGNIFICANT_DIGITS: i32 = 3;

impl Price {
    pub const ZERO: Price = Price(0);

    pub fn to_cent(self) -> u64 {
        self.0
    }

    pub fn to_dollar(self) -> f64 {
        self.0 as f64 / 100.0
    }

    pub fn from_cent(cent: u64) -> Self {
        Self(cent)
    }

    pub fn from_dollar(dollar: f64) -> Self {
        Self((dollar * 100.0) as u64)
    }

    pub fn to_significant(self) -> Self {
        let mut price = self;

        let ilog10 = price.0.checked_ilog10().unwrap_or(0) as i32;

        if ilog10 >= SIGNIFICANT_DIGITS {
            let log_diff = ilog10 - SIGNIFICANT_DIGITS + 1;

            let pow = 10.0_f64.powi(log_diff);

            price = Price::from_cent(((price.0 as f64 / pow).round() * pow) as u64);
        }

        price
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Price {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Price {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<Amount> for Price {
    type Output = Self;

    fn mul(self, rhs: Amount) -> Self::Output {
        Self((self.to_cent() as f64 * rhs.to_sat() as f64 / Amount::ONE_BTC_F64).round() as u64)
    }
}

impl Div<Amount> for Price {
    type Output = Self;

    fn div(self, rhs: Amount) -> Self::Output {
        Self((self.to_cent() as f64 * Amount::ONE_BTC_F64 / rhs.to_sat() as f64).round() as u64)
    }
}
