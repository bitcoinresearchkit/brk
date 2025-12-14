use schemars::JsonSchema;
use serde::Serialize;

/// A single block size data point.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockSizeEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    pub avg_size: u64,
}
