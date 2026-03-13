use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco};

use crate::VSize;

/// Transaction or block weight in weight units (WU)
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
pub struct Weight(u64);

impl Weight {
    /// Maximum block weight in Bitcoin (4 million weight units).
    /// Note: Pre-SegWit 1MB blocks have weight = size * 4 = 4M, so this is consistent across all blocks.
    pub const MAX_BLOCK: Self = Self(4_000_000);

    /// Compute weight from base size and total size.
    /// Formula: weight = base_size * 3 + total_size
    /// (since total_size = base_size + witness_size, this equals base_size * 4 + witness_size)
    #[inline]
    pub fn from_sizes(base_size: u32, total_size: u32) -> Self {
        let wu = base_size as u64 * 3 + total_size as u64;
        Self(wu)
    }

    pub fn to_vbytes_ceil(&self) -> u64 {
        bitcoin::Weight::from(*self).to_vbytes_ceil()
    }

    pub fn to_vbytes_floor(&self) -> u64 {
        bitcoin::Weight::from(*self).to_vbytes_floor()
    }

    /// Returns block fullness as a ratio (0–1+) relative to MAX_BLOCK.
    #[inline]
    pub fn fullness(&self) -> f32 {
        (self.0 as f64 / Self::MAX_BLOCK.0 as f64) as f32
    }
}

impl From<bitcoin::Weight> for Weight {
    #[inline]
    fn from(value: bitcoin::Weight) -> Self {
        Self(value.to_wu())
    }
}

impl From<Weight> for bitcoin::Weight {
    #[inline]
    fn from(value: Weight) -> Self {
        Self::from_wu(value.0)
    }
}

impl From<VSize> for Weight {
    /// Convert virtual bytes to weight units: `weight = vbytes * WITNESS_SCALE_FACTOR`.
    #[inline]
    fn from(vsize: VSize) -> Self {
        Self(bitcoin::Weight::from_vb_unchecked(*vsize).to_wu())
    }
}

impl From<usize> for Weight {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<f64> for Weight {
    #[inline]
    fn from(value: f64) -> Self {
        let value = value.max(0.0);
        Self(value as u64)
    }
}

impl From<Weight> for f64 {
    #[inline]
    fn from(value: Weight) -> Self {
        value.0 as f64
    }
}

impl Add for Weight {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Weight {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Weight {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Weight {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Div<usize> for Weight {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(self.0 as usize / rhs)
    }
}

impl Div<Weight> for Weight {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl CheckedSub for Weight {
    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Weight {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
