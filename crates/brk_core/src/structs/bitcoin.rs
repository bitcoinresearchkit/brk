use std::ops::{Add, Div, Mul};

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Sats;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct Bitcoin(f64);

impl Add for Bitcoin {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(Sats::from(self) + Sats::from(rhs))
    }
}

impl Mul for Bitcoin {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(Sats::from(self) * Sats::from(rhs))
    }
}

impl Div<usize> for Bitcoin {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(Sats::from(self) / rhs)
    }
}

impl From<Sats> for Bitcoin {
    fn from(value: Sats) -> Self {
        Self(f64::from(value) / (f64::from(Sats::ONE_BTC)))
    }
}

impl From<f64> for Bitcoin {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Bitcoin> for f64 {
    fn from(value: Bitcoin) -> Self {
        value.0
    }
}

impl From<usize> for Bitcoin {
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl Eq for Bitcoin {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Bitcoin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
