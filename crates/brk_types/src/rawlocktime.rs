use bitcoin::{absolute::LockTime, locktime::absolute::LOCK_TIME_THRESHOLD};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, Pco};

/// Transaction locktime
#[derive(Debug, Clone, Copy, Serialize, Pco, JsonSchema)]
pub struct RawLockTime(u32);

impl From<LockTime> for RawLockTime {
    #[inline]
    fn from(value: LockTime) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<RawLockTime> for LockTime {
    #[inline]
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

impl Formattable for RawLockTime {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}
