use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{unlikely, CheckedSub, Formattable, Pco};

use super::StoredF32;

/// Signed basis points stored as i32.
/// 1 bp = 0.0001. Range: -214,748.3647 to +214,748.3647.
/// Use for unbounded signed values (returns, growth rates, volatility, z-scores, etc.).
/// `i32::MIN` is reserved as a NaN sentinel.
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
pub struct BasisPointsSigned32(i32);

impl BasisPointsSigned32 {
    pub const ZERO: Self = Self(0);
    /// NaN sentinel — uses i32::MIN which is outside the documented range.
    pub const NAN: Self = Self(i32::MIN);

    #[inline]
    pub const fn new(value: i32) -> Self {
        debug_assert!(value != i32::MIN, "i32::MIN is reserved as NaN sentinel");
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> i32 {
        self.0
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0 == i32::MIN
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0 && self.0 != i32::MIN
    }

    /// Convert to f32: divide by 10000. Returns NaN for sentinel value.
    #[inline]
    pub fn to_f32(self) -> f32 {
        if unlikely(self.0 == i32::MIN) {
            f32::NAN
        } else {
            self.0 as f32 / 10000.0
        }
    }
}

impl From<usize> for BasisPointsSigned32 {
    #[inline]
    fn from(value: usize) -> Self {
        debug_assert!(
            value <= i32::MAX as usize,
            "usize out of BasisPointsSigned32 range: {value}"
        );
        Self(value as i32)
    }
}

impl From<i32> for BasisPointsSigned32 {
    #[inline]
    fn from(value: i32) -> Self {
        debug_assert!(value != i32::MIN, "i32::MIN is reserved as NaN sentinel");
        Self(value)
    }
}

impl From<BasisPointsSigned32> for i32 {
    #[inline]
    fn from(value: BasisPointsSigned32) -> Self {
        value.0
    }
}

/// Convert from float: multiply by 10000 and round.
/// Input is in ratio form (e.g., 50.0 for +5000%).
/// NaN/Inf → NaN sentinel.
impl From<f64> for BasisPointsSigned32 {
    #[inline]
    fn from(value: f64) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0)
            .round()
            .clamp(i32::MIN as f64 + 1.0, i32::MAX as f64);
        Self(scaled as i32)
    }
}

/// Convert from f32 ratio form: multiply by 10000 and round.
/// Input is in ratio form (e.g., 0.5 for +50% → 5000 bps).
/// NaN/Inf → NaN sentinel.
impl From<f32> for BasisPointsSigned32 {
    #[inline]
    fn from(value: f32) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0)
            .round()
            .clamp(i32::MIN as f32 + 1.0, i32::MAX as f32);
        Self(scaled as i32)
    }
}

impl From<BasisPointsSigned32> for f64 {
    #[inline]
    fn from(value: BasisPointsSigned32) -> Self {
        if unlikely(value.0 == i32::MIN) {
            f64::NAN
        } else {
            value.0 as f64 / 10000.0
        }
    }
}

impl From<BasisPointsSigned32> for StoredF32 {
    #[inline]
    fn from(value: BasisPointsSigned32) -> Self {
        StoredF32::from(value.to_f32())
    }
}

impl Add for BasisPointsSigned32 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == i32::MIN || rhs.0 == i32::MIN) {
            Self::NAN
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

impl AddAssign for BasisPointsSigned32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for BasisPointsSigned32 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == i32::MIN || rhs.0 == i32::MIN) {
            Self::NAN
        } else {
            Self(self.0 - rhs.0)
        }
    }
}

impl SubAssign for BasisPointsSigned32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Div<usize> for BasisPointsSigned32 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        if unlikely(self.0 == i32::MIN) {
            Self::NAN
        } else {
            debug_assert!(rhs <= i32::MAX as usize, "divisor out of i32 range: {rhs}");
            Self(self.0 / rhs as i32)
        }
    }
}

impl CheckedSub for BasisPointsSigned32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if unlikely(self.0 == i32::MIN || rhs.0 == i32::MIN) {
            Some(Self::NAN)
        } else {
            self.0.checked_sub(rhs.0).map(Self)
        }
    }
}

impl std::fmt::Display for BasisPointsSigned32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for BasisPointsSigned32 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }

    #[inline(always)]
    fn fmt_json(&self, buf: &mut Vec<u8>) {
        if unlikely(self.0 == i32::MIN) {
            buf.extend_from_slice(b"null");
        } else {
            self.write_to(buf);
        }
    }
}
