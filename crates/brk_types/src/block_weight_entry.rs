use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single block weight data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockWeightEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    pub avg_weight: u64,
}
