use schemars::JsonSchema;
use serde::Serialize;

use crate::{BlockHash, Height, Timestamp, Weight};

/// Block information returned by the API
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BlockInfo {
    /// Block hash
    pub id: BlockHash,

    /// Block height
    pub height: Height,

    /// Number of transactions in the block
    pub tx_count: u32,

    /// Block size in bytes
    pub size: u64,

    /// Block weight in weight units
    pub weight: Weight,

    /// Block timestamp (Unix time)
    pub timestamp: Timestamp,

    /// Block difficulty as a floating point number
    pub difficulty: f64,
}
