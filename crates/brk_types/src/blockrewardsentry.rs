use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single block rewards data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockRewardsEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    pub avg_rewards: u64,
}
