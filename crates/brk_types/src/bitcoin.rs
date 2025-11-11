use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul},
};

use serde::Serialize;
use vecdb::{CheckedSub, Compressable, Formattable};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Sats, StoredF64};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    Compressable,
)]
pub struct Bitcoin(f64);

impl Add for Bitcoin {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(Sats::from(self) + Sats::from(rhs))
    }
}

impl AddAssign for Bitcoin {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul for Bitcoin {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(Sats::from(self) * Sats::from(rhs))
    }
}

impl Mul<usize> for Bitcoin {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self::from(Sats::from(self) * rhs)
    }
}

impl Div<Bitcoin> for Bitcoin {
    type Output = StoredF64;
    fn div(self, rhs: Bitcoin) -> Self::Output {
        StoredF64::from(self.0 / rhs.0)
        // Self::from(Sats::from(self) / Sats::from(rhs))
    }
}

impl Div<usize> for Bitcoin {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(Sats::from(self) / rhs)
    }
}

impl From<Sats> for Bitcoin {
    #[inline]
    fn from(value: Sats) -> Self {
        Self(f64::from(value) / (f64::from(Sats::ONE_BTC)))
    }
}

impl From<f64> for Bitcoin {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<StoredF64> for Bitcoin {
    #[inline]
    fn from(value: StoredF64) -> Self {
        Self(*value)
    }
}

impl From<Bitcoin> for f64 {
    #[inline]
    fn from(value: Bitcoin) -> Self {
        value.0
    }
}

impl From<usize> for Bitcoin {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl PartialEq for Bitcoin {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for Bitcoin {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for Bitcoin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Bitcoin {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}

impl CheckedSub<usize> for Bitcoin {
    fn checked_sub(self, rhs: usize) -> Option<Self> {
        Some(Self(self.0 - rhs as f64))
    }
}
impl CheckedSub<Bitcoin> for Bitcoin {
    fn checked_sub(self, rhs: Bitcoin) -> Option<Self> {
        Some(Self(self.0 - rhs.0))
    }
}

impl std::fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = ryu::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Bitcoin {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
