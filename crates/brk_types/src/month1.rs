use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Date, Day1, Year1};

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
pub struct Month1(u16);

impl From<u16> for Month1 {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Month1> for u16 {
    #[inline]
    fn from(value: Month1) -> Self {
        value.0
    }
}

impl From<usize> for Month1 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Month1> for u64 {
    #[inline]
    fn from(value: Month1) -> Self {
        value.0 as u64
    }
}

impl From<Month1> for usize {
    #[inline]
    fn from(value: Month1) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Month1 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Add<Month1> for Month1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Month1 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Month1 {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<Day1> for Month1 {
    #[inline]
    fn from(value: Day1) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Month1 {
    #[inline]
    fn from(value: Date) -> Self {
        Self(u8::from(Year1::from(value)) as u16 * 12 + value.month() as u16 - 1)
    }
}

impl CheckedSub for Month1 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for Month1 {
    fn to_string() -> &'static str {
        "month1"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["month", "m", "monthly", "month1", "monthindex"]
    }
}

impl std::fmt::Display for Month1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Month1 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
