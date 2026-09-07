use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use crate::CheckedSub;
use brk_error::{Error, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "storage")]
use vecdb::{Formattable, Pco, PrintableIndex};

use super::{Date, Day1, Month1, Timestamp};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "storage", derive(Pco))]
pub struct Year1(u8);

impl Year1 {
    pub fn to_timestamp(&self) -> Timestamp {
        Timestamp::from(Date::from(*self))
    }
}

impl From<u8> for Year1 {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Year1 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Year1> for u64 {
    #[inline]
    fn from(value: Year1) -> Self {
        value.0 as u64
    }
}

impl From<Year1> for usize {
    #[inline]
    fn from(value: Year1) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Year1 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl Add<Year1> for Year1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Year1 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Year1 {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<Day1> for Year1 {
    #[inline]
    fn from(value: Day1) -> Self {
        Self::from(usize::from(Date::from(value).year() - 2009))
    }
}

impl TryFrom<Date> for Year1 {
    type Error = Error;

    #[inline]
    fn try_from(value: Date) -> Result<Self> {
        value.try_into_jiff()?;
        let years = value
            .year()
            .checked_sub(2009)
            .ok_or(Error::UnindexableDate)?;
        u8::try_from(years)
            .map(Self)
            .map_err(|_| Error::UnindexableDate)
    }
}

impl From<Year1> for u8 {
    #[inline]
    fn from(value: Year1) -> Self {
        value.0
    }
}

impl CheckedSub for Year1 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
#[cfg(feature = "storage")]
impl vecdb::CheckedSub for Year1 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        crate::CheckedSub::checked_sub(self, rhs)
    }
}

impl From<Month1> for Year1 {
    #[inline]
    fn from(value: Month1) -> Self {
        Self((usize::from(value) / 12) as u8)
    }
}

impl Year1 {
    pub fn index_name() -> &'static str {
        "year1"
    }
    pub fn index_aliases() -> &'static [&'static str] {
        &["1y", "y", "year", "yearly", "year1", "yearindex"]
    }
}
#[cfg(feature = "storage")]
impl PrintableIndex for Year1 {
    fn to_string() -> &'static str {
        Self::index_name()
    }
    fn to_possible_strings() -> &'static [&'static str] {
        Self::index_aliases()
    }
}

impl std::fmt::Display for Year1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

#[cfg(feature = "storage")]
impl Formattable for Year1 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
