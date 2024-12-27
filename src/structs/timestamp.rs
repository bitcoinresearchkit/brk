use std::{fmt, ops::Sub};

use allocative::Allocative;
use bincode::{Decode, Encode};
use chrono::{NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::utils::{ONE_DAY_IN_S, ONE_HOUR_IN_S};

use super::{Date, HeightMapChunkId, MapKey};

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

    pub fn now() -> Self {
        Self(chrono::offset::Utc::now().timestamp() as u32)
    }

    pub fn to_date(self) -> Date {
        Date::wrap(
            Utc.timestamp_opt(i64::from(self.0), 0)
                .unwrap()
                .date_naive(),
        )
    }

    pub fn to_floored_seconds(self) -> Self {
        let date_time = Utc.timestamp_opt(i64::from(self.0), 0).unwrap();

        Self::from(
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
        (*younger).checked_sub(**self).unwrap_or_default() > ONE_HOUR_IN_S as u32
    }
}

impl Sub for Timestamp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.0 - rhs.0)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl MapKey<HeightMapChunkId> for Timestamp {
    fn to_chunk_id(&self) -> HeightMapChunkId {
        unreachable!();
    }

    fn to_first_unsafe(&self) -> Option<Self> {
        unreachable!();
    }

    fn to_serialized_key(&self) -> Self {
        unreachable!();
    }

    fn is_out_of_bounds(&self) -> bool {
        unreachable!();
    }

    fn is_first(&self) -> bool {
        unreachable!();
    }

    fn checked_sub(&self, _: usize) -> Option<Self> {
        unreachable!();
    }

    fn min_percentile_key() -> Self {
        unreachable!();
    }

    fn iter_up_to(&self, other: &Self) -> impl Iterator<Item = Self> {
        (**self..=**other).map(Timestamp::from)
    }

    fn map_name<'a>() -> &'a str {
        "timestamp"
    }

    fn to_usize(&self) -> usize {
        (**self) as usize
    }

    fn from_usize(t: usize) -> Self {
        Self(t as u32)
    }
}
