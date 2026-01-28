use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use super::Dollars;

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

    #[inline]
    pub const fn inner(self) -> u64 {
        self.0
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
