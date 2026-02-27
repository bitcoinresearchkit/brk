use std::ops::Add;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{Timestamp, INDEX_EPOCH};

pub const DAY3_INTERVAL: u32 = 259200;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct Day3(u16);

impl Day3 {
    pub fn from_timestamp(ts: Timestamp) -> Self {
        Self(((*ts - INDEX_EPOCH + 86400) / DAY3_INTERVAL) as u16)
    }

    pub fn to_timestamp(&self) -> Timestamp {
        Timestamp::new(INDEX_EPOCH - 86400 + self.0 as u32 * DAY3_INTERVAL)
    }
}

impl From<Day3> for usize {
    #[inline]
    fn from(value: Day3) -> Self {
        value.0 as usize
    }
}

impl From<usize> for Day3 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl Add<usize> for Day3 {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl CheckedSub for Day3 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for Day3 {
    fn to_string() -> &'static str {
        "day3"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["3d", "day3"]
    }
}

impl std::fmt::Display for Day3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Day3 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
