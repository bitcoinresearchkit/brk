use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use super::{Sats, VSize};

/// Fee rate in sats/vB
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Pco, JsonSchema)]
pub struct FeeRate(f64);

impl FeeRate {
    pub const MIN: Self = Self(0.1);

    pub fn new(fr: f64) -> Self {
        Self(fr)
    }
}

impl From<(Sats, VSize)> for FeeRate {
    #[inline]
    fn from((sats, vsize): (Sats, VSize)) -> Self {
        if sats.is_zero() {
            return Self(0.0);
        }
        let sats = u64::from(sats);
        let vsize = u64::from(vsize);
        if vsize == 0 {
            return Self(f64::NAN);
        }
        Self((sats * 1000).div_ceil(vsize) as f64 / 1000.0)
    }
}

impl From<f64> for FeeRate {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<FeeRate> for f64 {
    #[inline]
    fn from(value: FeeRate) -> Self {
        value.0
    }
}

impl Add for FeeRate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for FeeRate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Div<usize> for FeeRate {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            Self(f64::NAN)
        } else {
            Self(self.0 / rhs as f64)
        }
    }
}

impl From<usize> for FeeRate {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl PartialEq for FeeRate {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for FeeRate {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl PartialOrd for FeeRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for FeeRate {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}

impl std::fmt::Display for FeeRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = ryu::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for FeeRate {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
