use std::ops::Sub;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Cents, Dollars};

/// Compact unsigned cents (u32) - memory-efficient for map keys.
/// Supports values from $0.00 to $42,949,672.95 (u32::MAX / 100).
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
    JsonSchema,
)]
pub struct CentsCompact(u32);

impl CentsCompact {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(u32::MAX);

    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn inner(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub const fn as_u128(self) -> u128 {
        self.0 as u128
    }

    #[inline]
    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Round to N significant digits.
    /// E.g., 12345 (= $123.45) with round_to(4) → 12350 (= $123.50)
    /// E.g., 12345 (= $123.45) with round_to(3) → 12300 (= $123.00)
    #[inline]
    pub fn round_to(self, digits: i32) -> Self {
        let v = self.0;
        let ilog10 = v.checked_ilog10().unwrap_or(0) as i32;
        if ilog10 >= digits {
            let log_diff = ilog10 - digits + 1;
            let pow = 10u32.pow(log_diff as u32);
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
            let pow = 10u32.pow(log_diff as u32);
            ((dollars + pow / 2) / pow) * pow
        } else {
            dollars
        };
        Self(rounded_dollars * 100)
    }
}

impl From<Dollars> for CentsCompact {
    #[inline]
    fn from(value: Dollars) -> Self {
        let f = f64::from(value);
        if f.is_nan() || f < 0.0 {
            Self::ZERO
        } else {
            let cents = (f * 100.0).round();
            debug_assert!(
                cents <= u32::MAX as f64,
                "Price ${} exceeds CentsUnsignedCompact max (~$42.9M)",
                f
            );
            Self(cents as u32)
        }
    }
}

impl From<CentsCompact> for Dollars {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        value.to_dollars()
    }
}

impl From<u32> for CentsCompact {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<CentsCompact> for u32 {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        value.0
    }
}

impl From<CentsCompact> for f64 {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        value.0 as f64
    }
}

impl From<Cents> for CentsCompact {
    #[inline]
    fn from(value: Cents) -> Self {
        let v = value.inner();
        debug_assert!(
            v <= u32::MAX as u64,
            "CentsUnsigned {} exceeds CentsUnsignedCompact max",
            v
        );
        Self(v as u32)
    }
}

impl From<CentsCompact> for Cents {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        Cents::new(value.0 as u64)
    }
}

impl Sub for CentsCompact {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::fmt::Display for CentsCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
