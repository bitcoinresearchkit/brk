use schemars::JsonSchema;
use serde::Serialize;

/// A single block fees data point.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeesEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    pub avg_fees: u64,
}
