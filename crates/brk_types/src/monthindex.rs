use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

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
)]
pub struct MonthIndex(u16);

impl From<u16> for MonthIndex {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<MonthIndex> for u16 {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        value.0
    }
}

impl From<usize> for MonthIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<MonthIndex> for u64 {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        value.0 as u64
    }
}

impl From<MonthIndex> for usize {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for MonthIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<MonthIndex> for MonthIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for MonthIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for MonthIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<DateIndex> for MonthIndex {
    #[inline]
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for MonthIndex {
    #[inline]
    fn from(value: Date) -> Self {
        Self(u16::from(YearIndex::from(value)) * 12 + value.month() as u16 - 1)
    }
}

impl CheckedSub for MonthIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for MonthIndex {
    fn to_string() -> &'static str {
        "monthindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["m", "month", "monthindex"]
    }
}

impl std::fmt::Display for MonthIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
