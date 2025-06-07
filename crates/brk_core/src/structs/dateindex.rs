use std::{
    fmt,
    ops::{Add, Rem},
};

use serde::Serialize;
// use color_eyre::eyre::eyre;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Error};

use super::Date;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct DateIndex(u16);

impl DateIndex {
    pub const BYTES: usize = size_of::<Self>();
}

impl From<DateIndex> for usize {
    fn from(value: DateIndex) -> Self {
        value.0 as usize
    }
}

impl From<usize> for DateIndex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<DateIndex> for i64 {
    fn from(value: DateIndex) -> Self {
        value.0 as i64
    }
}

impl Add<usize> for DateIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl TryFrom<Date> for DateIndex {
    type Error = Error;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let value_ = jiff::civil::Date::from(value);
        if value_ < Date::INDEX_ZERO_ {
            Err(Error::UnindexableDate)
        } else if value == Date::INDEX_ZERO {
            Ok(Self(0))
        } else if value_ < Date::INDEX_ONE_ {
            Err(Error::UnindexableDate)
        } else if value == Date::INDEX_ONE {
            Ok(Self(1))
        } else {
            Ok(Self(Date::INDEX_ONE_.until(value_)?.get_days() as u16 + 1))
        }
    }
}

impl CheckedSub for DateIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Rem<usize> for DateIndex {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self(self.0 % rhs as u16)
    }
}

impl fmt::Display for DateIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
