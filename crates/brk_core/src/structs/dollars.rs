use std::ops::{Add, Div, Mul};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Bitcoin, Cents, Sats};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Deref,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Dollars(f64);

impl From<f64> for Dollars {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Cents> for Dollars {
    fn from(value: Cents) -> Self {
        Self(f64::from(value) / 100.0)
    }
}

impl From<Dollars> for f64 {
    fn from(value: Dollars) -> Self {
        value.0
    }
}

impl From<usize> for Dollars {
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl Add for Dollars {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(Cents::from(self) + Cents::from(rhs))
    }
}

impl Div<usize> for Dollars {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(Cents::from(self) / rhs)
    }
}

impl Eq for Dollars {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Dollars {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Mul<Bitcoin> for Dollars {
    type Output = Dollars;
    fn mul(self, rhs: Bitcoin) -> Self::Output {
        Self::from(Cents::from(
            u64::from(Sats::from(rhs)) * u64::from(Cents::from(self)) / u64::from(Sats::ONE_BTC),
        ))
    }
}
