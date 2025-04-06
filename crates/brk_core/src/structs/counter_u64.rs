use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Txinindex, Txoutindex};

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
pub struct CounterU64(u64);

impl CounterU64 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u64) -> Self {
        Self(counter)
    }
}

impl From<u64> for CounterU64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for CounterU64 {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl CheckedSub<CounterU64> for CounterU64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for CounterU64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl Add for CounterU64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for CounterU64 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u64)
    }
}

impl From<CounterU64> for f64 {
    fn from(value: CounterU64) -> Self {
        value.0 as f64
    }
}

impl From<Txinindex> for CounterU64 {
    fn from(value: Txinindex) -> Self {
        Self(*value)
    }
}

impl From<Txoutindex> for CounterU64 {
    fn from(value: Txoutindex) -> Self {
        Self(*value)
    }
}
