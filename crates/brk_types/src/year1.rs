use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Date, Day1, Month1};

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
pub struct Year1(u8);

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
        Self::from(Date::from(value))
    }
}

impl From<Date> for Year1 {
    #[inline]
    fn from(value: Date) -> Self {
        Self((value.year() - 2009) as u8)
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

impl From<Month1> for Year1 {
    #[inline]
    fn from(value: Month1) -> Self {
        Self((usize::from(value) / 12) as u8)
    }
}

impl PrintableIndex for Year1 {
    fn to_string() -> &'static str {
        "year1"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["1y", "y", "year", "yearly", "year1", "yearindex"]
    }
}

impl std::fmt::Display for Year1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Year1 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
