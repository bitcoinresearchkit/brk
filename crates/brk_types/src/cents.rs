use std::ops::{Add, Div, Mul};

use serde::Serialize;
use vecdb::{CheckedSub, Formattable, Pco};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
    Pco,
)]
pub struct Cents(i64);

impl Cents {
    pub const fn mint(value: i64) -> Self {
        Self(value)
    }

    pub fn round_to(self, digits: i32) -> Self {
        let v = self.0;

        let ilog10 = v.checked_ilog10().unwrap_or(0) as i32;

        Self::from(if ilog10 >= digits {
            let log_diff = ilog10 - digits + 1;

            let pow = 10.0_f64.powi(log_diff);

            ((v as f64 / pow).round() * pow) as i64
        } else {
            v
        })
    }
}

impl From<Dollars> for Cents {
    #[inline]
    fn from(value: Dollars) -> Self {
        Self((*value * 100.0).round() as i64)
    }
}

impl From<Cents> for f64 {
    #[inline]
    fn from(value: Cents) -> Self {
        value.0 as f64
    }
}

impl From<i64> for Cents {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for Cents {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl From<Cents> for usize {
    #[inline]
    fn from(value: Cents) -> Self {
        if value.0 < 0 {
            panic!()
        }
        value.0 as usize
    }
}

impl From<usize> for Cents {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as i64)
    }
}

impl From<Cents> for i64 {
    #[inline]
    fn from(value: Cents) -> Self {
        value.0
    }
}

impl From<Cents> for u64 {
    #[inline]
    fn from(value: Cents) -> Self {
        if value.0 < 0 {
            panic!("Shouldn't convert neg cents to u64")
        }
        value.0 as u64
    }
}

impl Add for Cents {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<Cents> for Cents {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<usize> for Cents {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i64)
    }
}

impl From<u128> for Cents {
    #[inline]
    fn from(value: u128) -> Self {
        if value > i64::MAX as u128 {
            panic!("u128 bigger than i64")
        }
        Self(value as i64)
    }
}

impl From<Cents> for u128 {
    #[inline]
    fn from(value: Cents) -> Self {
        if value.0 < 0 {
            panic!("Shouldn't convert neg cents to u128")
        }
        value.0 as u128
    }
}

impl Mul<Cents> for Cents {
    type Output = Cents;
    fn mul(self, rhs: Cents) -> Self::Output {
        Self(self.0.checked_mul(rhs.0).unwrap())
    }
}

impl Mul<i64> for Cents {
    type Output = Cents;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<usize> for Cents {
    type Output = Cents;
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as i64)
    }
}

impl CheckedSub for Cents {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Cents::from)
    }
}

impl std::fmt::Display for Cents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Cents {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
