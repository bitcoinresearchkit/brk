use std::ops::{Add, AddAssign, Div};

use derive_deref::Deref;
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
pub struct StoredI16(i16);

impl StoredI16 {
    pub const ZERO: Self = Self(0);

    pub fn new(v: i16) -> Self {
        Self(v)
    }
}

impl From<i16> for StoredI16 {
    #[inline]
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredI16 {
    #[inline]
    fn from(value: usize) -> Self {
        if value > i16::MAX as usize {
            panic!("usize too big (value = {value})")
        }
        Self(value as i16)
    }
}

impl CheckedSub<StoredI16> for StoredI16 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredI16 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i16)
    }
}

impl Add for StoredI16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredI16 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<f64> for StoredI16 {
    #[inline]
    fn from(value: f64) -> Self {
        if value < 0.0 || value > i16::MAX as f64 {
            panic!()
        }
        Self(value as i16)
    }
}

impl From<StoredI16> for f64 {
    #[inline]
    fn from(value: StoredI16) -> Self {
        value.0 as f64
    }
}

impl From<StoredI16> for usize {
    #[inline]
    fn from(value: StoredI16) -> Self {
        value.0 as usize
    }
}

impl PrintableIndex for StoredI16 {
    fn to_string() -> &'static str {
        "i16"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["i16"]
    }
}

impl std::fmt::Display for StoredI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for StoredI16 {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
