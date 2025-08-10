use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Printable, StoredCompressed};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::MonthIndex;

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
pub struct QuarterIndex(u16);

impl From<u16> for QuarterIndex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for QuarterIndex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<QuarterIndex> for u16 {
    fn from(value: QuarterIndex) -> Self {
        value.0
    }
}

impl From<QuarterIndex> for usize {
    fn from(value: QuarterIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for QuarterIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<QuarterIndex> for QuarterIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for QuarterIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for QuarterIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<MonthIndex> for QuarterIndex {
    fn from(value: MonthIndex) -> Self {
        Self((usize::from(value) / 3) as u16)
    }
}

impl CheckedSub for QuarterIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for QuarterIndex {
    fn to_string() -> &'static str {
        "quarterindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["q", "quarter", "quarterindex"]
    }
}
