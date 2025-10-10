use bitcoin::{absolute::LockTime, locktime::absolute::LOCK_TIME_THRESHOLD};
use serde::Serialize;
use vecdb::StoredCompressed;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug, Immutable, Clone, Copy, IntoBytes, KnownLayout, FromBytes, Serialize, StoredCompressed,
)]
pub struct RawLockTime(u32);

impl From<LockTime> for RawLockTime {
    fn from(value: LockTime) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<RawLockTime> for LockTime {
    fn from(value: RawLockTime) -> Self {
        let value = value.0;
        if value < LOCK_TIME_THRESHOLD {
            bitcoin::locktime::absolute::Height::from_consensus(value)
                .unwrap()
                .into()
        } else {
            bitcoin::locktime::absolute::Time::from_consensus(value)
                .unwrap()
                .into()
        }
    }
}

impl std::fmt::Display for RawLockTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lock_time = LockTime::from(*self);
        f.write_str(&lock_time.to_string())
    }
}
