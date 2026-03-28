use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, unlikely};

use super::StoredF32;

/// Signed basis points stored as i16.
/// 1 bp = 0.0001. Range: -3.2767 to +3.2767.
/// Use for signed bounded ratios (NUPL, net PnL ratios, etc.).
/// `i16::MIN` is reserved as a NaN sentinel.
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
pub struct BasisPointsSigned16(i16);

impl BasisPointsSigned16 {
    pub const ZERO: Self = Self(0);
    /// NaN sentinel — uses i16::MIN which is outside the documented range.
    pub const NAN: Self = Self(i16::MIN);

    #[inline]
    pub const fn new(value: i16) -> Self {
        debug_assert!(value != i16::MIN, "i16::MIN is reserved as NaN sentinel");
        Self(value)
    }

    #[inline(always)]
    pub const fn inner(self) -> i16 {
        self.0
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0 == i16::MIN
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0 && self.0 != i16::MIN
    }

    /// Convert to f32: divide by 10000. Returns NaN for sentinel value.
    #[inline]
    pub fn to_f32(self) -> f32 {
        if unlikely(self.0 == i16::MIN) {
            f32::NAN
        } else {
            self.0 as f32 / 10000.0
        }
    }
}

impl From<usize> for BasisPointsSigned16 {
    #[inline]
    fn from(value: usize) -> Self {
        debug_assert!(
            value <= i16::MAX as usize,
            "usize out of BasisPointsSigned16 range: {value}"
        );
        Self(value as i16)
    }
}

impl From<i16> for BasisPointsSigned16 {
    #[inline]
    fn from(value: i16) -> Self {
        debug_assert!(value != i16::MIN, "i16::MIN is reserved as NaN sentinel");
        Self(value)
    }
}

impl From<BasisPointsSigned16> for i16 {
    #[inline]
    fn from(value: BasisPointsSigned16) -> Self {
        value.0
    }
}

/// Convert from float: multiply by 10000 and round.
/// Input is in ratio form (e.g., -0.4523 for -45.23%).
/// NaN/Inf → NaN sentinel.
impl From<f64> for BasisPointsSigned16 {
    #[inline]
    fn from(value: f64) -> Self {
        if unlikely(!value.is_finite()) {
            return Self::NAN;
        }
        let scaled = (value * 10000.0)
            .round()
            .clamp(i16::MIN as f64 + 1.0, i16::MAX as f64);
        Self(scaled as i16)
    }
}

impl From<BasisPointsSigned16> for f64 {
    #[inline]
    fn from(value: BasisPointsSigned16) -> Self {
        if unlikely(value.0 == i16::MIN) {
            f64::NAN
        } else {
            value.0 as f64 / 10000.0
        }
    }
}

impl From<BasisPointsSigned16> for StoredF32 {
    #[inline]
    fn from(value: BasisPointsSigned16) -> Self {
        StoredF32::from(value.to_f32())
    }
}

impl Add for BasisPointsSigned16 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == i16::MIN || rhs.0 == i16::MIN) {
            Self::NAN
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

impl AddAssign for BasisPointsSigned16 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for BasisPointsSigned16 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        if unlikely(self.0 == i16::MIN || rhs.0 == i16::MIN) {
            Self::NAN
        } else {
            Self(self.0 - rhs.0)
        }
    }
}

impl SubAssign for BasisPointsSigned16 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Div<usize> for BasisPointsSigned16 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        if unlikely(self.0 == i16::MIN) {
            Self::NAN
        } else {
            debug_assert!(rhs <= i16::MAX as usize, "divisor out of i16 range: {rhs}");
            Self(self.0 / rhs as i16)
        }
    }
}

impl CheckedSub for BasisPointsSigned16 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if unlikely(self.0 == i16::MIN || rhs.0 == i16::MIN) {
            Some(Self::NAN)
        } else {
            self.0.checked_sub(rhs.0).map(Self)
        }
    }
}

impl std::fmt::Display for BasisPointsSigned16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for BasisPointsSigned16 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }

    #[inline(always)]
    fn fmt_json(&self, buf: &mut Vec<u8>) {
        if unlikely(self.0 == i16::MIN) {
            buf.extend_from_slice(b"null");
        } else {
            self.write_to(buf);
        }
    }
}
