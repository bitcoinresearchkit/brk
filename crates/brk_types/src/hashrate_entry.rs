use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Timestamp;

/// A single hashrate data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HashrateEntry {
    /// Unix timestamp.
    pub timestamp: Timestamp,
    /// Average hashrate (H/s).
    #[serde(rename = "avgHashrate")]
    pub avg_hashrate: u128,
}
