use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
pub struct Halvingepoch(u8);

impl From<u8> for Halvingepoch {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Halvingepoch {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Halvingepoch> for usize {
    fn from(value: Halvingepoch) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Halvingepoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}
