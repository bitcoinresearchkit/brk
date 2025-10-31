use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use allocative::Allocative;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Date, DateIndex, MonthIndex};

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
pub struct YearIndex(u16);

impl From<u16> for YearIndex {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for YearIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<YearIndex> for u64 {
    #[inline]
    fn from(value: YearIndex) -> Self {
        value.0 as u64
    }
}

impl From<YearIndex> for usize {
    #[inline]
    fn from(value: YearIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for YearIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<YearIndex> for YearIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for YearIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for YearIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<DateIndex> for YearIndex {
    #[inline]
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for YearIndex {
    #[inline]
    fn from(value: Date) -> Self {
        Self(value.year() - 2009)
    }
}

impl From<YearIndex> for u16 {
    #[inline]
    fn from(value: YearIndex) -> Self {
        value.0
    }
}

impl CheckedSub for YearIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<MonthIndex> for YearIndex {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        Self((usize::from(value) / 12) as u16)
    }
}

impl PrintableIndex for YearIndex {
    fn to_string() -> &'static str {
        "yearindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["y", "year", "yearindex"]
    }
}

impl std::fmt::Display for YearIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
