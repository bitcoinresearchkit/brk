use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use allocative::Allocative;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Printable, StoredCompressed};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Height;

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
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    StoredCompressed,
    Allocative,
)]
pub struct HalvingEpoch(u16);

impl HalvingEpoch {
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

impl From<u16> for HalvingEpoch {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for HalvingEpoch {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<HalvingEpoch> for usize {
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
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Height> for HalvingEpoch {
    fn from(value: Height) -> Self {
        Self((u32::from(value) / 210_000) as u16)
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

impl Printable for HalvingEpoch {
    fn to_string() -> &'static str {
        "halvingepoch"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["halving", "halvingepoch"]
    }
}
