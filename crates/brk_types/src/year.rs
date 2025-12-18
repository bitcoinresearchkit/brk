use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Date, Timestamp};

/// Bitcoin year (2009, 2010, ..., 2025+)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Serialize, Deserialize, Pco,
)]
pub struct Year(u16);

impl Year {
    pub const GENESIS: Self = Self(2009);

    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the year as an index (0 = 2009, 1 = 2010, etc.)
    pub fn to_index(self) -> usize {
        (self.0 - 2009) as usize
    }
}

impl From<u16> for Year {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for Year {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Year> for usize {
    #[inline]
    fn from(value: Year) -> Self {
        value.0 as usize
    }
}

impl From<Year> for u16 {
    #[inline]
    fn from(value: Year) -> Self {
        value.0
    }
}

impl Add for Year {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for Year {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<usize> for Year {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Timestamp> for Year {
    #[inline]
    fn from(value: Timestamp) -> Self {
        Self(Date::from(value).year())
    }
}

impl From<Date> for Year {
    #[inline]
    fn from(value: Date) -> Self {
        Self(value.year())
    }
}

impl CheckedSub for Year {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for Year {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(self.0 as usize / rhs)
    }
}

impl PrintableIndex for Year {
    fn to_string() -> &'static str {
        "year"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["year"]
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Year {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
