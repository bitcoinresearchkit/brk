use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Content hash of the projected next block (block 0 of the mempool
/// snapshot). Same value as the mempool ETag. Opaque token: pass back
/// as `since` on `/api/v1/mining/block-template/diff/{hash}` to fetch
/// deltas.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(transparent)]
pub struct NextBlockHash(u64);

impl NextBlockHash {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl fmt::Display for NextBlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for NextBlockHash {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<NextBlockHash> for u64 {
    fn from(value: NextBlockHash) -> Self {
        value.0
    }
}
