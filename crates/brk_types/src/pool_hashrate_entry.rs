use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Timestamp;

/// A single pool hashrate data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolHashrateEntry {
    /// Unix timestamp.
    pub timestamp: Timestamp,
    /// Average hashrate (H/s).
    #[serde(rename = "avgHashrate")]
    pub avg_hashrate: u128,
    /// Pool's share of total network hashrate.
    pub share: f64,
    /// Pool name.
    #[serde(rename = "poolName")]
    pub pool_name: String,
}
