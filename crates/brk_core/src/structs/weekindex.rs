use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

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
)]
pub struct WeekIndex(u16);

impl From<u16> for WeekIndex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<WeekIndex> for u16 {
    fn from(value: WeekIndex) -> Self {
        value.0
    }
}

impl From<usize> for WeekIndex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<WeekIndex> for usize {
    fn from(value: WeekIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for WeekIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<DateIndex> for WeekIndex {
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for WeekIndex {
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

impl Printable for WeekIndex {
    fn to_string() -> &'static str {
        "weekindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["w", "week", "weekindex"]
    }
}
