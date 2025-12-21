use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use crate::Weight;

/// Virtual size in vbytes (weight / 4, rounded up)
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
pub struct VSize(u64);

impl VSize {
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<u64> for VSize {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<VSize> for u64 {
    #[inline]
    fn from(value: VSize) -> Self {
        value.0
    }
}

impl From<Weight> for VSize {
    #[inline]
    fn from(weight: Weight) -> Self {
        Self(weight.to_vbytes_ceil())
    }
}

impl From<usize> for VSize {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<f64> for VSize {
    #[inline]
    fn from(value: f64) -> Self {
        debug_assert!(
            value >= 0.0 && value.fract() == 0.0,
            "VSize must be a non-negative integer"
        );
        Self(value as u64)
    }
}

impl From<VSize> for f64 {
    #[inline]
    fn from(value: VSize) -> Self {
        value.0 as f64
    }
}

impl Add for VSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for VSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for VSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for VSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Div<usize> for VSize {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl std::fmt::Display for VSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for VSize {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
