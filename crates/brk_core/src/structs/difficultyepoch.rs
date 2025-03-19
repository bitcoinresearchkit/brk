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
pub struct Difficultyepoch(u16);

impl From<u16> for Difficultyepoch {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for Difficultyepoch {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Difficultyepoch> for usize {
    fn from(value: Difficultyepoch) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Difficultyepoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}
