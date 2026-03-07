use std::ops::{Add, AddAssign, Div};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco};

use super::StoredF32;

/// Unsigned basis points stored as u32.
/// 1 bp = 0.0001. Range: 0–429,496.7295.
/// Use for unbounded unsigned ratios (MVRV, NVT, SOPR, etc.).
#[derive(
    Debug,
    Deref,
    Clone,
    Default,
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
pub struct BasisPoints32(u32);

impl BasisPoints32 {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> u32 {
        self.0
    }

    /// Convert to f32: divide by 10000.
    #[inline]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 10000.0
    }
}

impl From<usize> for BasisPoints32 {
    #[inline]
    fn from(value: usize) -> Self {
        debug_assert!(
            value <= u32::MAX as usize,
            "usize out of BasisPoints32 range: {value}"
        );
        Self(value as u32)
    }
}

impl From<u32> for BasisPoints32 {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<BasisPoints32> for u32 {
    #[inline]
    fn from(value: BasisPoints32) -> Self {
        value.0
    }
}

/// Convert from float: multiply by 10000 and round.
/// Input is in ratio form (e.g., 2.5 for MVRV of 2.5).
impl From<f64> for BasisPoints32 {
    #[inline]
    fn from(value: f64) -> Self {
        let scaled = (value * 10000.0).round().clamp(0.0, u32::MAX as f64);
        Self(scaled as u32)
    }
}

impl From<BasisPoints32> for f64 {
    #[inline]
    fn from(value: BasisPoints32) -> Self {
        value.0 as f64 / 10000.0
    }
}

impl From<BasisPoints32> for StoredF32 {
    #[inline]
    fn from(value: BasisPoints32) -> Self {
        StoredF32::from(value.to_f32())
    }
}

impl Add for BasisPoints32 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for BasisPoints32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Div<usize> for BasisPoints32 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        debug_assert!(rhs <= u32::MAX as usize, "divisor out of u32 range: {rhs}");
        Self(self.0 / rhs as u32)
    }
}

impl CheckedSub for BasisPoints32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl std::fmt::Display for BasisPoints32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for BasisPoints32 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
