use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Date, Day1};

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
    Pco,
    JsonSchema,
)]
pub struct Week1(u16);

impl From<u16> for Week1 {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Week1> for u16 {
    #[inline]
    fn from(value: Week1) -> Self {
        value.0
    }
}

impl From<usize> for Week1 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Week1> for usize {
    #[inline]
    fn from(value: Week1) -> Self {
        value.0 as usize
    }
}

impl Add<Week1> for Week1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Week1 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Week1 {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl Add<usize> for Week1 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Day1> for Week1 {
    #[inline]
    fn from(value: Day1) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Week1 {
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

impl CheckedSub for Week1 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for Week1 {
    fn to_string() -> &'static str {
        "week1"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["1w", "w", "week", "weekly", "week1", "weekindex"]
    }
}

impl std::fmt::Display for Week1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Week1 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
