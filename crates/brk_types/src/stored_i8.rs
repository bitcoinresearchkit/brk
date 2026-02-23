use std::ops::{Add, AddAssign, Div};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

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
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct StoredI8(i8);

impl StoredI8 {
    pub const ZERO: Self = Self(0);

    pub fn new(v: i8) -> Self {
        Self(v)
    }
}

impl From<i8> for StoredI8 {
    #[inline]
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredI8 {
    #[inline]
    fn from(value: usize) -> Self {
        if value > i8::MAX as usize {
            panic!("usize too big (value = {value})")
        }
        Self(value as i8)
    }
}

impl CheckedSub<StoredI8> for StoredI8 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredI8 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i8)
    }
}

impl Add for StoredI8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredI8 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<f64> for StoredI8 {
    #[inline]
    fn from(value: f64) -> Self {
        if value < i8::MIN as f64 || value > i8::MAX as f64 {
            panic!()
        }
        Self(value as i8)
    }
}

impl From<StoredI8> for f64 {
    #[inline]
    fn from(value: StoredI8) -> Self {
        value.0 as f64
    }
}

impl From<StoredI8> for usize {
    #[inline]
    fn from(value: StoredI8) -> Self {
        value.0 as usize
    }
}

impl PrintableIndex for StoredI8 {
    fn to_string() -> &'static str {
        "i8"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["i8"]
    }
}

impl std::fmt::Display for StoredI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for StoredI8 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
