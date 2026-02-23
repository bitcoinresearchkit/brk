use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use super::Dollars;

/// Signed cents (i64) - for values that can be negative.
/// Used for profit/loss calculations, deltas, etc.
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
    Pco,
    JsonSchema,
)]
pub struct CentsSigned(i64);

impl CentsSigned {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn inner(self) -> i64 {
        self.0
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }

    pub fn round_to(self, digits: i32) -> Self {
        let v = self.0;
        let ilog10 = v.unsigned_abs().checked_ilog10().unwrap_or(0) as i32;
        Self::from(if ilog10 >= digits {
            let log_diff = ilog10 - digits + 1;
            let pow = 10.0_f64.powi(log_diff);
            ((v as f64 / pow).round() * pow) as i64
        } else {
            v
        })
    }
}

impl From<Dollars> for CentsSigned {
    #[inline]
    fn from(value: Dollars) -> Self {
        Self((*value * 100.0).round() as i64)
    }
}

impl From<CentsSigned> for Dollars {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        value.to_dollars()
    }
}

impl From<CentsSigned> for f64 {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        value.0 as f64
    }
}

impl From<i64> for CentsSigned {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for CentsSigned {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl From<CentsSigned> for usize {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        debug_assert!(value.0 >= 0, "Cannot convert negative CentsSigned to usize");
        value.0 as usize
    }
}

impl From<usize> for CentsSigned {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as i64)
    }
}

impl From<CentsSigned> for i64 {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        value.0
    }
}

impl From<CentsSigned> for u64 {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        debug_assert!(value.0 >= 0, "Cannot convert negative CentsSigned to u64");
        value.0 as u64
    }
}

impl Add for CentsSigned {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for CentsSigned {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for CentsSigned {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for CentsSigned {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Div<CentsSigned> for CentsSigned {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<usize> for CentsSigned {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i64)
    }
}

impl From<CentsSigned> for i128 {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        value.0 as i128
    }
}

impl From<i128> for CentsSigned {
    #[inline]
    fn from(value: i128) -> Self {
        debug_assert!(
            value >= i64::MIN as i128 && value <= i64::MAX as i128,
            "i128 overflow to CentsSigned"
        );
        Self(value as i64)
    }
}

impl From<u128> for CentsSigned {
    #[inline]
    fn from(value: u128) -> Self {
        debug_assert!(value <= i64::MAX as u128, "u128 overflow to CentsSigned");
        Self(value as i64)
    }
}

impl From<CentsSigned> for u128 {
    #[inline]
    fn from(value: CentsSigned) -> Self {
        debug_assert!(value.0 >= 0, "Cannot convert negative CentsSigned to u128");
        value.0 as u128
    }
}

impl Mul<CentsSigned> for CentsSigned {
    type Output = CentsSigned;
    #[inline]
    fn mul(self, rhs: CentsSigned) -> Self::Output {
        Self(self.0.checked_mul(rhs.0).expect("CentsSigned overflow"))
    }
}

impl Mul<i64> for CentsSigned {
    type Output = CentsSigned;
    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<usize> for CentsSigned {
    type Output = CentsSigned;
    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as i64)
    }
}

impl std::fmt::Display for CentsSigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for CentsSigned {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
