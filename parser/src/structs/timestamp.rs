use std::ops::Sub;

use allocative::Allocative;
use bincode::{Decode, Encode};
use chrono::{Datelike, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::utils::{ONE_DAY_IN_S, ONE_HOUR_IN_S};

use super::Date;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Allocative,
    Serialize,
    Deserialize,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Encode,
    Decode,
)]
pub struct Timestamp(u32);

impl Timestamp {
    pub const ZERO: Self = Self(0);

    pub fn wrap(timestamp: u32) -> Self {
        Self(timestamp)
    }

    pub fn to_date(self) -> Date {
        Date::wrap(
            Utc.timestamp_opt(i64::from(self.0), 0)
                .unwrap()
                .date_naive(),
        )
    }

    pub fn to_year(self) -> u32 {
        self.to_date().year() as u32
    }

    pub fn to_floored_seconds(self) -> Self {
        let date_time = Utc.timestamp_opt(i64::from(self.0), 0).unwrap();

        Self::wrap(
            NaiveDateTime::new(
                date_time.date_naive(),
                NaiveTime::from_hms_opt(date_time.hour(), date_time.minute(), 0).unwrap(),
            )
            .and_utc()
            .timestamp() as u32,
        )
    }

    pub fn difference_in_days_between(older: Self, younger: Self) -> u32 {
        if younger <= older {
            0
        } else {
            *(younger - older) / ONE_DAY_IN_S as u32
        }
    }

    pub fn older_by_1h_plus_than(&self, younger: Self) -> bool {
        younger.checked_sub(**self).unwrap_or_default() > ONE_HOUR_IN_S as u32
    }
}

impl Sub for Timestamp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::wrap(self.0 - rhs.0)
    }
}
