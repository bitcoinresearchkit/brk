use derive_deref::Deref;
use jiff::{civil::date, tz::TimeZone};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize,
)]
pub struct Timestamp(u32);

impl Timestamp {
    pub fn floor_seconds(self) -> Self {
        let t = jiff::Timestamp::from(self).to_zoned(TimeZone::UTC);
        let d = jiff::civil::DateTime::from(t);
        let d = date(d.year(), d.month(), d.day()).at(d.hour(), d.minute(), 0, 0);
        Self::from(d.to_zoned(TimeZone::UTC).unwrap().timestamp())
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

impl From<Timestamp> for bitcoin::locktime::absolute::Time {
    fn from(value: Timestamp) -> Self {
        bitcoin::locktime::absolute::Time::from_consensus(*value).unwrap()
    }
}
