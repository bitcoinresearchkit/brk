use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::PoolStats;

/// Mining pools response for a time period
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PoolsSummary {
    /// List of pools sorted by block count descending
    pub pools: Vec<PoolStats>,
    /// Total blocks in the time period
    pub block_count: u64,
    /// Estimated network hashrate (hashes per second)
    pub last_estimated_hashrate: u128,
    /// Estimated network hashrate over last 3 days
    pub last_estimated_hashrate3d: u128,
    /// Estimated network hashrate over last 1 week
    pub last_estimated_hashrate1w: u128,
}
