use std::ops::Add;

use crate::CheckedSub;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "storage")]
use vecdb::{Formattable, Pco, PrintableIndex};

use super::{INDEX_EPOCH, Timestamp};

pub const HOUR12_INTERVAL: u32 = 43200;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "storage", derive(Pco))]
pub struct Hour12(u32);

impl Hour12 {
    pub fn from_timestamp(ts: Timestamp) -> Self {
        Self((*ts - INDEX_EPOCH) / HOUR12_INTERVAL)
    }

    pub fn to_timestamp(&self) -> Timestamp {
        Timestamp::new(INDEX_EPOCH + self.0 * HOUR12_INTERVAL)
    }
}

impl From<Hour12> for usize {
    #[inline]
    fn from(value: Hour12) -> Self {
        value.0 as usize
    }
}

impl From<usize> for Hour12 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Add<usize> for Hour12 {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl CheckedSub for Hour12 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
#[cfg(feature = "storage")]
impl vecdb::CheckedSub for Hour12 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        crate::CheckedSub::checked_sub(self, rhs)
    }
}

impl Hour12 {
    pub fn index_name() -> &'static str {
        "hour12"
    }
    pub fn index_aliases() -> &'static [&'static str] {
        &["12h", "hour12"]
    }
}
#[cfg(feature = "storage")]
impl PrintableIndex for Hour12 {
    fn to_string() -> &'static str {
        Self::index_name()
    }
    fn to_possible_strings() -> &'static [&'static str] {
        Self::index_aliases()
    }
}

impl std::fmt::Display for Hour12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

#[cfg(feature = "storage")]
impl Formattable for Hour12 {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
