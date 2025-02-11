use serde::Serialize;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::{Height, Timestamp};

#[derive(Debug, Immutable, Clone, Copy, IntoBytes, KnownLayout, TryFromBytes, Serialize)]
#[repr(C)]
pub enum LockTime {
    Height(Height),
    Timestamp(Timestamp),
}

impl From<bitcoin::absolute::LockTime> for LockTime {
    fn from(value: bitcoin::absolute::LockTime) -> Self {
        match value {
            bitcoin::absolute::LockTime::Blocks(h) => LockTime::Height(h.into()),
            bitcoin::absolute::LockTime::Seconds(t) => LockTime::Timestamp(t.into()),
        }
    }
}

impl From<LockTime> for bitcoin::absolute::LockTime {
    fn from(value: LockTime) -> Self {
        match value {
            LockTime::Height(h) => bitcoin::absolute::LockTime::Blocks(h.into()),
            LockTime::Timestamp(t) => bitcoin::absolute::LockTime::Seconds(t.into()),
        }
    }
}
