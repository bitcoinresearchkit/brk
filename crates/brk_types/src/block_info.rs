use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{BlockHash, BlockHeader, Height, Timestamp, Weight};

/// Block information matching mempool.space /api/block/{hash}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockInfo {
    /// Block hash
    pub id: BlockHash,

    /// Block height
    pub height: Height,

    /// Block header fields
    #[serde(flatten)]
    pub header: BlockHeader,

    /// Block timestamp (Unix time)
    pub timestamp: Timestamp,

    /// Number of transactions in the block
    pub tx_count: u32,

    /// Block size in bytes
    pub size: u64,

    /// Block weight in weight units
    pub weight: Weight,

    /// Median time of the last 11 blocks
    #[serde(rename = "mediantime")]
    pub median_time: Timestamp,

    /// Block difficulty
    pub difficulty: f64,
}
