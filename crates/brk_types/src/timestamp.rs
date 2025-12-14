use std::ops::{Add, AddAssign, Div};

use derive_deref::Deref;
use jiff::{civil::date, tz::TimeZone};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco};

use super::Date;

/// Timestamp
#[derive(
    Debug,
    Deref,
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
pub struct Timestamp(u32);

pub const ONE_HOUR_IN_SEC: u32 = 60 * 60;
pub const ONE_DAY_IN_SEC: u32 = 24 * 60 * 60;
pub const ONE_DAY_IN_SEC_F64: f64 = ONE_DAY_IN_SEC as f64;

impl Timestamp {
    pub const ZERO: Self = Self(0);

    pub fn new(timestamp: u32) -> Self {
        Self(timestamp)
    }

    pub fn floor_seconds(self) -> Self {
        let zoned = jiff::Timestamp::from(self).to_zoned(TimeZone::UTC);
        let date_time = jiff::civil::DateTime::from(zoned);
        let trunc_date_time = date(date_time.year(), date_time.month(), date_time.day()).at(
            date_time.hour(),
            date_time.minute(),
            0,
            0,
        );
        Self::from(trunc_date_time.to_zoned(TimeZone::UTC).unwrap().timestamp())
    }

    #[inline]
    pub fn difference_in_days_between(&self, older: Self) -> usize {
        // if self.0 < older.0 {
        //     unreachable!()
        // }
        ((self.0 - older.0) / ONE_DAY_IN_SEC) as usize
    }

    #[inline]
    pub fn difference_in_days_between_float(&self, older: Self) -> f64 {
        // if self.0 < older.0 {
        //     unreachable!()
        // }
        (self.0 - older.0) as f64 / ONE_DAY_IN_SEC_F64
    }

    #[inline]
    pub fn is_more_than_hour(&self) -> bool {
        self.0 >= ONE_HOUR_IN_SEC
    }

    pub fn now() -> Self {
        Self::from(jiff::Timestamp::now())
    }
}

impl From<u32> for Timestamp {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<jiff::Timestamp> for Timestamp {
    #[inline]
    fn from(value: jiff::Timestamp) -> Self {
        Self(value.as_second() as u32)
    }
}

impl From<Timestamp> for jiff::Timestamp {
    #[inline]
    fn from(value: Timestamp) -> Self {
        jiff::Timestamp::from_second(*value as i64).unwrap()
    }
}

impl From<bitcoin::locktime::absolute::Time> for Timestamp {
    #[inline]
    fn from(value: bitcoin::locktime::absolute::Time) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<usize> for Timestamp {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Date> for Timestamp {
    #[inline]
    fn from(value: Date) -> Self {
        Self::from(
            jiff::civil::Date::from(value)
                .to_zoned(TimeZone::UTC)
                .unwrap()
                .timestamp(),
        )
    }
}

impl CheckedSub<Timestamp> for Timestamp {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for Timestamp {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u32)
    }
}

impl Add for Timestamp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<f64> for Timestamp {
    #[inline]
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u32)
    }
}

impl From<Timestamp> for f64 {
    #[inline]
    fn from(value: Timestamp) -> Self {
        value.0 as f64
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Timestamp {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
