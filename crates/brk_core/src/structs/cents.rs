use std::ops::{Add, Div};

use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Dollars;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Cents(u64);

impl From<Dollars> for Cents {
    fn from(value: Dollars) -> Self {
        Self((*value * 100.0).round() as u64)
    }
}

impl From<Cents> for f64 {
    fn from(value: Cents) -> Self {
        value.0 as f64
    }
}

impl From<u64> for Cents {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Cents> for u64 {
    fn from(value: Cents) -> Self {
        value.0
    }
}

impl Add for Cents {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Cents {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}
