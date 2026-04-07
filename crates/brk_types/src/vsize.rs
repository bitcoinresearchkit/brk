use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco};

use crate::Weight;

/// Virtual size in vbytes (weight / 4, rounded up). Max block vsize is ~1,000,000 vB.
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
#[schemars(example = 110, example = 140, example = 225, example = 500_000, example = 998_368)]
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
        let value = value.max(0.0);
        debug_assert!(value.fract() == 0.0, "VSize must be an integer");
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

impl CheckedSub for VSize {
    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
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
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
