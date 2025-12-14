use schemars::JsonSchema;
use serde::Serialize;

/// A single block weight data point.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockWeightEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    pub avg_weight: u64,
}
