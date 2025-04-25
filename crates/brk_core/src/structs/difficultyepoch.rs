use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
pub struct DifficultyEpoch(u16);

impl From<u16> for DifficultyEpoch {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for DifficultyEpoch {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<DifficultyEpoch> for usize {
    fn from(value: DifficultyEpoch) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for DifficultyEpoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Height> for DifficultyEpoch {
    fn from(value: Height) -> Self {
        Self((u32::from(value) / 2016) as u16)
    }
}

impl CheckedSub for DifficultyEpoch {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
