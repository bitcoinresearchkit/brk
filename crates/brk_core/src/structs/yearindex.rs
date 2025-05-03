use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

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
)]
pub struct YearIndex(u8);

impl From<u8> for YearIndex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for YearIndex {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<YearIndex> for usize {
    fn from(value: YearIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for YearIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<DateIndex> for YearIndex {
    fn from(value: DateIndex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for YearIndex {
    fn from(value: Date) -> Self {
        Self((value.year() - 2009) as u8)
    }
}

impl From<YearIndex> for u16 {
    fn from(value: YearIndex) -> Self {
        value.0 as u16
    }
}

impl CheckedSub for YearIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<MonthIndex> for YearIndex {
    fn from(value: MonthIndex) -> Self {
        Self((usize::from(value) / 12) as u8)
    }
}
