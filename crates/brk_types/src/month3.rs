use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Date, Month1, Timestamp};

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
pub struct Month3(u8);

impl Month3 {
    pub fn to_timestamp(&self) -> Timestamp {
        Timestamp::from(Date::from(*self))
    }
}

impl From<u8> for Month3 {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Month3 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Month3> for u8 {
    #[inline]
    fn from(value: Month3) -> Self {
        value.0
    }
}

impl From<Month3> for usize {
    #[inline]
    fn from(value: Month3) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Month3 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl Add<Month3> for Month3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Month3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Month3 {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<Month1> for Month3 {
    #[inline]
    fn from(value: Month1) -> Self {
        Self((usize::from(value) / 3) as u8)
    }
}

impl CheckedSub for Month3 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for Month3 {
    fn to_string() -> &'static str {
        "month3"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["quarter", "q", "quarterly", "month3", "quarterindex", "3m", "3mo"]
    }
}

impl std::fmt::Display for Month3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Month3 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
