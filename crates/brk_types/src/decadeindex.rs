use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use allocative::Allocative;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Date, DateIndex, YearIndex};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    StoredCompressed,
    Allocative,
)]
pub struct DecadeIndex(u16);

impl From<u16> for DecadeIndex {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<DecadeIndex> for u16 {
    #[inline]
    fn from(value: DecadeIndex) -> Self {
        value.0
    }
}

impl From<usize> for DecadeIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<DecadeIndex> for usize {
    #[inline]
    fn from(value: DecadeIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for DecadeIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<DecadeIndex> for DecadeIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for DecadeIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for DecadeIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<DateIndex> for DecadeIndex {
    #[inline]
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for DecadeIndex {
    #[inline]
    fn from(value: Date) -> Self {
        let year = value.year();
        if year < 2000 {
            panic!("unsupported")
        }
        Self((year - 2000) / 10)
    }
}

impl CheckedSub for DecadeIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<YearIndex> for DecadeIndex {
    #[inline]
    fn from(value: YearIndex) -> Self {
        let v = usize::from(value);
        if v == 0 {
            Self(0)
        } else {
            Self((((v - 1) / 10) + 1) as u16)
        }
    }
}

impl PrintableIndex for DecadeIndex {
    fn to_string() -> &'static str {
        "decadeindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["decade", "decadeindex"]
    }
}

impl std::fmt::Display for DecadeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
