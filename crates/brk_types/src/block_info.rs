use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{BlockHash, Height, Timestamp, Weight};

/// Block information matching mempool.space /api/block/{hash}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockInfo {
    /// Block hash
    pub id: BlockHash,
    /// Block height
    pub height: Height,
    /// Block version
    pub version: u32,
    /// Block timestamp (Unix time)
    pub timestamp: Timestamp,
    /// Number of transactions
    pub tx_count: u32,
    /// Block size in bytes
    pub size: u64,
    /// Block weight in weight units
    pub weight: Weight,
    /// Merkle root of the transaction tree
    pub merkle_root: String,
    /// Previous block hash
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: BlockHash,
    /// Median time of the last 11 blocks
    #[serde(rename = "mediantime")]
    pub median_time: Timestamp,
    /// Nonce
    pub nonce: u32,
    /// Compact target (bits)
    pub bits: u32,
    /// Block difficulty
    pub difficulty: f64,
}
