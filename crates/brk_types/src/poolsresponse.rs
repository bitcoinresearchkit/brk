use schemars::JsonSchema;
use serde::Serialize;

use crate::PoolStats;

/// Mining pools response for a time period
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolsResponse {
    /// List of pools sorted by block count descending
    pub pools: Vec<PoolStats>,

    /// Total blocks in the time period
    #[serde(rename = "blockCount")]
    pub block_count: u32,
}
