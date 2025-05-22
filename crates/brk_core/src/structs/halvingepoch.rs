use std::{
    fmt::Debug,
    ops::{Add, Div},
};

use serde::{Deserialize, Serialize};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

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
)]
pub struct HalvingEpoch(u8);

impl HalvingEpoch {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl From<u8> for HalvingEpoch {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for HalvingEpoch {
    fn from(value: usize) -> Self {
        Self(value as u8)
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

impl Add<usize> for HalvingEpoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<Height> for HalvingEpoch {
    fn from(value: Height) -> Self {
        Self((u32::from(value) / 210_000) as u8)
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
