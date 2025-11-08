use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Date, DateIndex};

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
pub struct WeekIndex(u16);

impl From<u16> for WeekIndex {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<WeekIndex> for u16 {
    #[inline]
    fn from(value: WeekIndex) -> Self {
        value.0
    }
}

impl From<usize> for WeekIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<WeekIndex> for usize {
    #[inline]
    fn from(value: WeekIndex) -> Self {
        value.0 as usize
    }
}

impl Add<WeekIndex> for WeekIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for WeekIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for WeekIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl Add<usize> for WeekIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<DateIndex> for WeekIndex {
    #[inline]
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for WeekIndex {
    #[inline]
    fn from(value: Date) -> Self {
        let date = jiff::civil::Date::from(value).iso_week_date();

        let mut week: u16 = 0;
        let mut year = 2009;

        while date.year() > year {
            let d = jiff::civil::Date::new(year, 6, 6).unwrap();
            let i = d.iso_week_date();
            let w = i.weeks_in_year();
            week += w as u16;
            year += 1;
        }

        week += date.week() as u16;

        week -= 1;

        Self(week)
    }
}

impl CheckedSub for WeekIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for WeekIndex {
    fn to_string() -> &'static str {
        "weekindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["w", "week", "weekindex"]
    }
}

impl std::fmt::Display for WeekIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
