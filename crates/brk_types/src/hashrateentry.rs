use schemars::JsonSchema;
use serde::Serialize;

use super::Timestamp;

/// A single hashrate data point.
#[derive(Debug, Serialize, JsonSchema)]
pub struct HashrateEntry {
    /// Unix timestamp.
    pub timestamp: Timestamp,
    /// Average hashrate (H/s).
    #[serde(rename = "avgHashrate")]
    pub avg_hashrate: u128,
}
