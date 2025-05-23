use bitcoin::absolute::LockTime;
use serde::Serialize;
use zerocopy_derive::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

#[derive(Debug, Immutable, Clone, Copy, IntoBytes, KnownLayout, TryFromBytes, Serialize)]
pub struct RawLockTime(u32);

impl From<LockTime> for RawLockTime {
    fn from(value: LockTime) -> Self {
        Self(value.to_consensus_u32())
    }
}

const CONSENSUS_DELIMITER: u32 = 500_000_000;

impl From<RawLockTime> for LockTime {
    fn from(value: RawLockTime) -> Self {
        let value = value.0;
        if value >= CONSENSUS_DELIMITER {
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
