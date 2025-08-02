use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div},
};

use brk_vecs::StoredCompressed;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Sats, StoredU64};

#[derive(
    Debug, Clone, Copy, Serialize, FromBytes, Immutable, IntoBytes, KnownLayout, StoredCompressed,
)]
pub struct Feerate(f32);

impl From<(Sats, StoredU64)> for Feerate {
    fn from((sats, vsize): (Sats, StoredU64)) -> Self {
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

impl AddAssign for Feerate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
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

impl PartialEq for Feerate {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for Feerate {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for Feerate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Feerate {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}
