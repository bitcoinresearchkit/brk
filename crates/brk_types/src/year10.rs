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
pub struct Year10(u8);

impl From<u8> for Year10 {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Year10> for u8 {
    #[inline]
    fn from(value: Year10) -> Self {
        value.0
    }
}

impl From<usize> for Year10 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Year10> for usize {
    #[inline]
    fn from(value: Year10) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Year10 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl Add<Year10> for Year10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Year10 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Year10 {
    type Output = Self;
    fn div(self, _: usize) -> Self::Output {
        unreachable!()
    }
}

impl From<Day1> for Year10 {
    #[inline]
    fn from(value: Day1) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Year10 {
    #[inline]
    fn from(value: Date) -> Self {
        let year = value.year();
        if year < 2000 {
            panic!("unsupported")
        }
        Self(((year - 2000) / 10) as u8)
    }
}

impl CheckedSub for Year10 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<Year1> for Year10 {
    #[inline]
    fn from(value: Year1) -> Self {
        let v = usize::from(value);
        if v == 0 {
            Self(0)
        } else {
            Self((((v - 1) / 10) + 1) as u8)
        }
    }
}

impl PrintableIndex for Year10 {
    fn to_string() -> &'static str {
        "year10"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["10y", "decade", "year10", "decadeindex"]
    }
}

impl std::fmt::Display for Year10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Year10 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
