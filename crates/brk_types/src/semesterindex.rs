use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, PrintableIndex, StoredCompressed};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
pub struct SemesterIndex(u16);

impl From<u16> for SemesterIndex {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for SemesterIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<SemesterIndex> for u16 {
    #[inline]
    fn from(value: SemesterIndex) -> Self {
        value.0
    }
}

impl From<SemesterIndex> for usize {
    #[inline]
    fn from(value: SemesterIndex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for SemesterIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<SemesterIndex> for SemesterIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for SemesterIndex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for SemesterIndex {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<MonthIndex> for SemesterIndex {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        Self((usize::from(value) / 6) as u16)
    }
}

impl CheckedSub for SemesterIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for SemesterIndex {
    fn to_string() -> &'static str {
        "semesterindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["s", "semester", "semesterindex"]
    }
}

impl std::fmt::Display for SemesterIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
