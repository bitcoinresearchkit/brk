use std::ops::{Add, AddAssign, Div};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{unlikely, CheckedSub, Formattable, Pco};

use super::StoredF32;

/// Unsigned basis points stored as u32.
/// 1 bp = 0.0001. Range: 0–429,496.7295.
/// Use for unbounded unsigned ratios (MVRV, NVT, SOPR, etc.).
/// `u32::MAX` is reserved as a NaN sentinel.
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
    /// NaN sentinel — uses u32::MAX which is outside the practical range.
    pub const NAN: Self = Self(u32::MAX);

    #[inline]
    pub const fn new(value: u32) -> Self {
        debug_assert!(value != u32::MAX, "u32::MAX is reserved as NaN sentinel");
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0 == u32::MAX
    }

    /// Convert to f32: divide by 10000. Returns NaN for sentinel value.
    #[inline]
    pub fn to_f32(self) -> f32 {
        if unlikely(self.0 == u32::MAX) {
            f32::NAN
        } else {
            self.0 as f32 / 10000.0
        }
    }
}

impl From<usize> for BasisPoints32 {
    #[inline]
    fn from(value: usize) -> Self {
        debug_assert!(
            value < u32::MAX as usize,
            "usize out of BasisPoints32 range: {value}"
        );
        Self(value as u32)
    }
}

impl From<u32> for BasisPoints32 {
    #[inline]
    fn from(value: u32) -> Self {
        debug_assert!(value != u32::MAX, "u32::MAX is reserved as NaN sentinel");
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
/// NaN/Inf → NaN sentinel.
impl From<f64> for BasisPoints32 {
    #[inline]
    fn from(value: f64) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0).round().clamp(0.0, u32::MAX as f64 - 1.0);
        Self(scaled as u32)
    }
}

impl From<BasisPoints32> for f64 {
    #[inline]
    fn from(value: BasisPoints32) -> Self {
        if unlikely(value.0 == u32::MAX) {
            f64::NAN
        } else {
            value.0 as f64 / 10000.0
        }
    }
}

impl From<f32> for BasisPoints32 {
    #[inline]
    fn from(value: f32) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        Self::from(value as f64)
    }
}

impl From<StoredF32> for BasisPoints32 {
    #[inline]
    fn from(value: StoredF32) -> Self {
        Self::from(f64::from(*value))
    }
}

impl From<BasisPoints32> for f32 {
    #[inline]
    fn from(value: BasisPoints32) -> Self {
        if unlikely(value.0 == u32::MAX) {
            f32::NAN
        } else {
            value.0 as f32 / 10000.0
        }
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
        if unlikely(self.0 == u32::MAX || rhs.0 == u32::MAX) {
            Self::NAN
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

impl AddAssign for BasisPoints32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Div<usize> for BasisPoints32 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        if unlikely(self.0 == u32::MAX) {
            Self::NAN
        } else {
            debug_assert!(rhs <= u32::MAX as usize, "divisor out of u32 range: {rhs}");
            Self(self.0 / rhs as u32)
        }
    }
}

impl CheckedSub for BasisPoints32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if unlikely(self.0 == u32::MAX || rhs.0 == u32::MAX) {
            Some(Self::NAN)
        } else {
            self.0.checked_sub(rhs.0).map(Self)
        }
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

    #[inline(always)]
    fn fmt_json(&self, buf: &mut Vec<u8>) {
        if unlikely(self.0 == u32::MAX) {
            buf.extend_from_slice(b"null");
        } else {
            self.write_to(buf);
        }
    }
}
