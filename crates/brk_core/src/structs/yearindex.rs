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
pub struct Yearindex(u8);

impl From<u8> for Yearindex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Yearindex {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Yearindex> for usize {
    fn from(value: Yearindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Yearindex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}
