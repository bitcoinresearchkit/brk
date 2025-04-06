use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

#[derive(
    Debug,
    Deref,
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
pub struct CounterU32(u32);

impl CounterU32 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u32) -> Self {
        Self(counter)
    }
}

impl From<u32> for CounterU32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for CounterU32 {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl CheckedSub<CounterU32> for CounterU32 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for CounterU32 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u32)
    }
}

impl Add for CounterU32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for CounterU32 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u32)
    }
}

impl From<CounterU32> for f64 {
    fn from(value: CounterU32) -> Self {
        value.0 as f64
    }
}
