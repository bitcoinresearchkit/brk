use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::PoolStats;

/// Mining pools response for a time period
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolsSummary {
    /// List of pools sorted by block count descending
    pub pools: Vec<PoolStats>,

    /// Total blocks in the time period
    #[serde(rename = "blockCount")]
    pub block_count: u32,

    /// Estimated network hashrate (hashes per second)
    #[serde(rename = "lastEstimatedHashrate")]
    pub last_estimated_hashrate: u128,
}
