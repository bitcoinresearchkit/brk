use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

/// Raw cents squared (u128) - stores cents² × sats without division.
/// Used for precise accumulation of investor cap values: Σ(price² × sats).
/// investor_price = investor_cap_raw / realized_cap_raw
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct CentsSquaredSats(u128);

impl CentsSquaredSats {
    pub const ZERO: Self = Self(0);

    #[inline(always)]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> u128 {
        self.0
    }
}

impl Div<u128> for CentsSquaredSats {
    type Output = u128;
    #[inline(always)]
    fn div(self, rhs: u128) -> u128 {
        self.0 / rhs
    }
}

impl AddAssign<u128> for CentsSquaredSats {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u128) {
        self.0 += rhs;
    }
}

impl SubAssign<u128> for CentsSquaredSats {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u128) {
        self.0 -= rhs;
    }
}

impl Add for CentsSquaredSats {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for CentsSquaredSats {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for CentsSquaredSats {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for CentsSquaredSats {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl From<u128> for CentsSquaredSats {
    #[inline(always)]
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<CentsSquaredSats> for u128 {
    #[inline(always)]
    fn from(value: CentsSquaredSats) -> Self {
        value.0
    }
}

impl From<usize> for CentsSquaredSats {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(value as u128)
    }
}

impl Div<usize> for CentsSquaredSats {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: usize) -> Self {
        Self(self.0 / rhs as u128)
    }
}

impl Formattable for CentsSquaredSats {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}

impl Bytes for CentsSquaredSats {
    type Array = [u8; 16];

    fn to_bytes(&self) -> Self::Array {
        self.0.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        Ok(Self(u128::from_bytes(bytes)?))
    }
}

impl std::fmt::Display for CentsSquaredSats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
