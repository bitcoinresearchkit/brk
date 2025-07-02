use std::{
    cmp::Ordering,
    ops::{Add, Div},
};

use derive_deref::Deref;
use jiff::{civil::date, tz::TimeZone};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::Date;

#[derive(
    Debug,
    Deref,
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
pub struct Timestamp(u32);

const ONE_DAY_IN_SEC: i64 = 24 * 60 * 60;

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

    pub fn difference_in_days_between(&self, other: Self) -> usize {
        match self.cmp(&other) {
            Ordering::Equal => 0,
            Ordering::Greater => other.difference_in_days_between(*self),
            Ordering::Less => {
                (jiff::Timestamp::from(*self)
                    .duration_until(jiff::Timestamp::from(other))
                    .as_secs()
                    / ONE_DAY_IN_SEC) as usize
            }
        }
    }

    pub fn difference_in_days_between_float(&self, other: Self) -> f64 {
        match self.cmp(&other) {
            Ordering::Equal => 0.0,
            Ordering::Greater => other.difference_in_days_between_float(*self),
            Ordering::Less => {
                jiff::Timestamp::from(*self)
                    .duration_until(jiff::Timestamp::from(other))
                    .as_secs() as f64
                    / ONE_DAY_IN_SEC as f64
            }
        }
    }

    pub fn is_more_than_hour(&self) -> bool {
        jiff::Timestamp::from(*self).as_second() >= 60 * 60
    }
}

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<jiff::Timestamp> for Timestamp {
    fn from(value: jiff::Timestamp) -> Self {
        Self(value.as_second() as u32)
    }
}

impl From<Timestamp> for jiff::Timestamp {
    fn from(value: Timestamp) -> Self {
        jiff::Timestamp::from_second(*value as i64).unwrap()
    }
}

impl From<bitcoin::locktime::absolute::Time> for Timestamp {
    fn from(value: bitcoin::locktime::absolute::Time) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<usize> for Timestamp {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Date> for Timestamp {
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

impl From<f64> for Timestamp {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u32)
    }
}

impl From<Timestamp> for f64 {
    fn from(value: Timestamp) -> Self {
        value.0 as f64
    }
}
