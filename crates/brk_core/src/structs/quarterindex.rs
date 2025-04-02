use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::Monthindex;

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
pub struct Quarterindex(u16);

impl From<u16> for Quarterindex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for Quarterindex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Quarterindex> for usize {
    fn from(value: Quarterindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Quarterindex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Monthindex> for Quarterindex {
    fn from(value: Monthindex) -> Self {
        Self((usize::from(value) / 3) as u16)
    }
}

impl CheckedSub for Quarterindex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
