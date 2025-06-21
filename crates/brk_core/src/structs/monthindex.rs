use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

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
)]
pub struct MonthIndex(u16);

impl From<u16> for MonthIndex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for MonthIndex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<MonthIndex> for usize {
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

impl From<DateIndex> for MonthIndex {
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for MonthIndex {
    fn from(value: Date) -> Self {
        Self(u16::from(YearIndex::from(value)) * 12 + value.month() as u16 - 1)
    }
}

impl CheckedSub for MonthIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for MonthIndex {
    fn to_string() -> &'static str {
        "monthindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["m", "month", "monthindex"]
    }
}
