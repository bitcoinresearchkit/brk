use derive_deref::Deref;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct Timestamp(u32);

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        Self(value)
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
