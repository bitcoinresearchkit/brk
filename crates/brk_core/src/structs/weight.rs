use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Immutable,
    IntoBytes,
    KnownLayout,
    FromBytes,
    Serialize,
)]
pub struct Weight(u64);

impl From<bitcoin::Weight> for Weight {
    fn from(value: bitcoin::Weight) -> Self {
        Self(value.to_wu())
    }
}

impl From<Weight> for bitcoin::Weight {
    fn from(value: Weight) -> Self {
        Self::from_wu(value.0)
    }
}

impl From<usize> for Weight {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<f64> for Weight {
    fn from(value: f64) -> Self {
        Self(value as u64)
    }
}

impl From<Weight> for f64 {
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
