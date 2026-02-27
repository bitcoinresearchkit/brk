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
pub struct HalvingEpoch(u8);

impl HalvingEpoch {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

impl From<u8> for HalvingEpoch {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for HalvingEpoch {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<HalvingEpoch> for usize {
    #[inline]
    fn from(value: HalvingEpoch) -> Self {
        value.0 as usize
    }
}

impl Add for HalvingEpoch {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for HalvingEpoch {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<usize> for HalvingEpoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<Height> for HalvingEpoch {
    #[inline]
    fn from(value: Height) -> Self {
        Self((u32::from(value) / BLOCKS_PER_HALVING) as u8)
    }
}

impl CheckedSub for HalvingEpoch {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for HalvingEpoch {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(self.0 as usize / rhs)
    }
}

impl PrintableIndex for HalvingEpoch {
    fn to_string() -> &'static str {
        "halvingepoch"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["halving", "halvingepoch", "halv"]
    }
}

impl std::fmt::Display for HalvingEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for HalvingEpoch {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}

impl From<f64> for HalvingEpoch {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value.round() as u8)
    }
}

impl From<HalvingEpoch> for f64 {
    #[inline]
    fn from(value: HalvingEpoch) -> Self {
        value.0 as f64
    }
}
