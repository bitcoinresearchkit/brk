use std::ops::{Add, AddAssign, Div, Sub};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{unlikely, CheckedSub, Formattable, Pco};

use super::StoredF32;

/// Unsigned basis points stored as u16.
/// 1 bp = 0.0001. Range: 0–6.5535.
/// Use for bounded 0–1 ratios (dominance, adoption, liveliness, etc.).
/// `u16::MAX` is reserved as a NaN sentinel.
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
pub struct BasisPoints16(u16);

impl BasisPoints16 {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(10000);
    /// NaN sentinel — uses u16::MAX which is outside the practical range.
    pub const NAN: Self = Self(u16::MAX);

    #[inline]
    pub const fn new(value: u16) -> Self {
        debug_assert!(value != u16::MAX, "u16::MAX is reserved as NaN sentinel");
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> u16 {
        self.0
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0 == u16::MAX
    }

    /// Convert to f32: divide by 10000. Returns NaN for sentinel value.
    #[inline]
    pub fn to_f32(self) -> f32 {
        if unlikely(self.0 == u16::MAX) {
            f32::NAN
        } else {
            self.0 as f32 / 10000.0
        }
    }
}

impl From<usize> for BasisPoints16 {
    #[inline]
    fn from(value: usize) -> Self {
        debug_assert!(
            value < u16::MAX as usize,
            "usize out of BasisPoints16 range: {value}"
        );
        Self(value as u16)
    }
}

impl From<u16> for BasisPoints16 {
    #[inline]
    fn from(value: u16) -> Self {
        debug_assert!(value != u16::MAX, "u16::MAX is reserved as NaN sentinel");
        Self(value)
    }
}

impl From<BasisPoints16> for u16 {
    #[inline]
    fn from(value: BasisPoints16) -> Self {
        value.0
    }
}

/// Convert from f32: multiply by 10000 and round.
/// Input is in ratio form (e.g., 0.4523 for 45.23%).
/// NaN/Inf → NaN sentinel.
impl From<f32> for BasisPoints16 {
    #[inline]
    fn from(value: f32) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0).round().clamp(0.0, u16::MAX as f32 - 1.0);
        Self(scaled as u16)
    }
}

/// Convert from f64: multiply by 10000 and round.
/// Input is in ratio form (e.g., 0.4523 for 45.23%).
/// NaN/Inf → NaN sentinel.
impl From<f64> for BasisPoints16 {
    #[inline]
    fn from(value: f64) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0).round().clamp(0.0, u16::MAX as f64 - 1.0);
        Self(scaled as u16)
    }
}

impl From<BasisPoints16> for f64 {
    #[inline]
    fn from(value: BasisPoints16) -> Self {
        if unlikely(value.0 == u16::MAX) {
            f64::NAN
        } else {
            value.0 as f64 / 10000.0
        }
    }
}

impl From<BasisPoints16> for StoredF32 {
    #[inline]
    fn from(value: BasisPoints16) -> Self {
        StoredF32::from(value.to_f32())
    }
}

impl Add for BasisPoints16 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == u16::MAX || rhs.0 == u16::MAX) {
            Self::NAN
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

impl Sub for BasisPoints16 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == u16::MAX || rhs.0 == u16::MAX) {
            Self::NAN
        } else {
            Self(self.0 - rhs.0)
        }
    }
}

impl AddAssign for BasisPoints16 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Div<usize> for BasisPoints16 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        if unlikely(self.0 == u16::MAX) {
            Self::NAN
        } else {
            debug_assert!(rhs <= u16::MAX as usize, "divisor out of u16 range: {rhs}");
            Self(self.0 / rhs as u16)
        }
    }
}

impl CheckedSub for BasisPoints16 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if unlikely(self.0 == u16::MAX || rhs.0 == u16::MAX) {
            Some(Self::NAN)
        } else {
            self.0.checked_sub(rhs.0).map(Self)
        }
    }
}

impl std::fmt::Display for BasisPoints16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for BasisPoints16 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }

    #[inline(always)]
    fn fmt_json(&self, buf: &mut Vec<u8>) {
        if unlikely(self.0 == u16::MAX) {
            buf.extend_from_slice(b"null");
        } else {
            self.write_to(buf);
        }
    }
}
