use schemars::JsonSchema;
use serde::Serialize;

use crate::Pool;

/// Mining pool with block statistics for a time period
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolStats {
    /// Pool information
    #[serde(flatten)]
    pub pool: &'static Pool,

    /// Number of blocks mined in the time period
    #[serde(rename = "blockCount")]
    pub block_count: u32,

    /// Pool's share of total blocks (0.0 - 1.0)
    pub share: f64,
}
