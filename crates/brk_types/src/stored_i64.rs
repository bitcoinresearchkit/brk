use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

/// Fixed-size 64-bit signed integer optimized for on-disk storage
#[derive(
    Debug,
    Default,
    Deref,
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
pub struct StoredI64(i64);

impl StoredI64 {
    pub const ZERO: Self = Self(0);

    pub fn new(v: i64) -> Self {
        Self(v)
    }
}

impl From<i64> for StoredI64 {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<StoredI64> for i64 {
    #[inline]
    fn from(value: StoredI64) -> Self {
        value.0
    }
}

impl From<usize> for StoredI64 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as i64)
    }
}

impl From<StoredI64> for usize {
    #[inline]
    fn from(value: StoredI64) -> Self {
        value.0 as usize
    }
}

impl CheckedSub<StoredI64> for StoredI64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredI64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i64)
    }
}

impl Add for StoredI64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredI64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for StoredI64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for StoredI64 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl From<f64> for StoredI64 {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value as i64)
    }
}

impl From<StoredI64> for f64 {
    #[inline]
    fn from(value: StoredI64) -> Self {
        value.0 as f64
    }
}

impl PrintableIndex for StoredI64 {
    fn to_string() -> &'static str {
        "i64"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["i64"]
    }
}

impl std::fmt::Display for StoredI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for StoredI64 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        f.push_str(buf.format(self.0));
        Ok(())
    }
}
