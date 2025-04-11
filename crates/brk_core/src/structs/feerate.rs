use std::ops::{Add, Div};

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Sats, StoredUsize};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    PartialEq,
    PartialOrd,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
pub struct Feerate(f32);

impl From<(Sats, StoredUsize)> for Feerate {
    fn from((sats, vsize): (Sats, StoredUsize)) -> Self {
        Self((f64::from(sats) / f64::from(vsize)) as f32)
    }
}

impl From<f64> for Feerate {
    fn from(value: f64) -> Self {
        Self(value as f32)
    }
}
impl From<Feerate> for f64 {
    fn from(value: Feerate) -> Self {
        value.0 as f64
    }
}

impl Add for Feerate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Feerate {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self((self.0 as f64 / rhs as f64) as f32)
    }
}

impl From<usize> for Feerate {
    fn from(value: usize) -> Self {
        Self(value as f32)
    }
}

impl Eq for Feerate {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Feerate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
