use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use super::{CentsSats, Dollars, Sats};

/// Unsigned cents (u64) - for values that should never be negative.
/// Used for invested capital, realized cap, etc.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct CentsUnsigned(u64);

impl CentsUnsigned {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(u64::MAX);

    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn as_u128(self) -> u128 {
        self.0 as u128
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }

    /// Round to N significant digits.
    /// E.g., 12345 (= $123.45) with round_to(4) → 12350 (= $123.50)
    /// E.g., 12345 (= $123.45) with round_to(3) → 12300 (= $123.00)
    pub fn round_to(self, digits: i32) -> Self {
        let v = self.0;
        let ilog10 = v.checked_ilog10().unwrap_or(0) as i32;
        if ilog10 >= digits {
            let log_diff = ilog10 - digits + 1;
            let pow = 10u64.pow(log_diff as u32);
            // Add half for rounding
            Self(((v + pow / 2) / pow) * pow)
        } else {
            self
        }
    }

    /// Round to nearest dollar, then apply N significant digits.
    /// E.g., 12345 (= $123.45) → 12300 (= $123.00) with 5 digits
    /// E.g., 1234567 (= $12345.67) → 1234600 (= $12346.00) with 5 digits
    #[inline]
    pub fn round_to_dollar(self, digits: i32) -> Self {
        // Round to nearest dollar (nearest 100 cents)
        let dollars = (self.0 + 50) / 100;
        // Apply significant digit rounding to dollars, then convert back to cents
        let ilog10 = dollars.checked_ilog10().unwrap_or(0) as i32;
        let rounded_dollars = if ilog10 >= digits {
            let log_diff = ilog10 - digits + 1;
            let pow = 10u64.pow(log_diff as u32);
            ((dollars + pow / 2) / pow) * pow
        } else {
            dollars
        };
        Self(rounded_dollars * 100)
    }
}

impl From<Dollars> for CentsUnsigned {
    #[inline]
    fn from(value: Dollars) -> Self {
        let f = f64::from(value);
        if f.is_nan() || f < 0.0 {
            Self::ZERO
        } else {
            Self((f * 100.0).round() as u64)
        }
    }
}

impl From<CentsUnsigned> for Dollars {
    #[inline]
    fn from(value: CentsUnsigned) -> Self {
        value.to_dollars()
    }
}

impl From<u64> for CentsUnsigned {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for CentsUnsigned {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<CentsUnsigned> for u64 {
    #[inline]
    fn from(value: CentsUnsigned) -> Self {
        value.0
    }
}

impl From<u128> for CentsUnsigned {
    #[inline]
    fn from(value: u128) -> Self {
        debug_assert!(value <= u64::MAX as u128, "u128 overflow to CentsUnsigned");
        Self(value as u64)
    }
}

impl From<CentsUnsigned> for u128 {
    #[inline]
    fn from(value: CentsUnsigned) -> Self {
        value.0 as u128
    }
}

impl From<CentsUnsigned> for f64 {
    #[inline]
    fn from(value: CentsUnsigned) -> Self {
        value.0 as f64
    }
}

impl From<f64> for CentsUnsigned {
    #[inline]
    fn from(value: f64) -> Self {
        if value.is_nan() || value < 0.0 {
            Self::ZERO
        } else {
            Self(value as u64)
        }
    }
}

impl Add for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for CentsUnsigned {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for CentsUnsigned {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_mul(rhs.0).expect("CentsUnsigned overflow"))
    }
}

impl Mul<u64> for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<usize> for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as u64)
    }
}

impl Mul<Sats> for CentsUnsigned {
    type Output = CentsSats;
    #[inline]
    fn mul(self, sats: Sats) -> CentsSats {
        CentsSats::new(self.as_u128() * sats.as_u128())
    }
}

impl Div<CentsUnsigned> for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<u64> for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<usize> for CentsUnsigned {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl std::fmt::Display for CentsUnsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for CentsUnsigned {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
