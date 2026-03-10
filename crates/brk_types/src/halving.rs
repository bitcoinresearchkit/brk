use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::Height;

pub const BLOCKS_PER_HALVING: u32 = 210_000;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct Halving(u8);

impl Halving {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

impl From<u8> for Halving {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Halving {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Halving> for usize {
    #[inline]
    fn from(value: Halving) -> Self {
        value.0 as usize
    }
}

impl Add for Halving {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Halving {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<usize> for Halving {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<Height> for Halving {
    #[inline]
    fn from(value: Height) -> Self {
        Self((u32::from(value) / BLOCKS_PER_HALVING) as u8)
    }
}

impl CheckedSub for Halving {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for Halving {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(self.0 as usize / rhs)
    }
}

impl PrintableIndex for Halving {
    fn to_string() -> &'static str {
        "halving"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["halving", "halvingepoch", "halv"]
    }
}

impl std::fmt::Display for Halving {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Halving {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}

impl From<f64> for Halving {
    #[inline]
    fn from(value: f64) -> Self {
        let value = value.max(0.0);
        Self(value.round() as u8)
    }
}

impl From<Halving> for f64 {
    #[inline]
    fn from(value: Halving) -> Self {
        value.0 as f64
    }
}
