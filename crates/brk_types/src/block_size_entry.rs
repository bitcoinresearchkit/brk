use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Height, Timestamp};

/// A single block size data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockSizeEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_size: u64,
}
