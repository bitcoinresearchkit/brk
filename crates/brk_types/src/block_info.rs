use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{BlockHash, Height, PoolSlug, Timestamp, Weight};

/// Block information returned by the API
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

    /// Extra block data (pool info, fee stats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<BlockExtras>,
}

/// Extra block data including pool identification and fee statistics
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockExtras {
    /// Mining pool that mined this block
    pub pool: BlockPool,

    /// Total fees in satoshis
    pub total_fees: u64,

    /// Average fee per transaction in satoshis
    pub avg_fee: u64,

    /// Average fee rate in sat/vB
    pub avg_fee_rate: u64,

    /// Total block reward (subsidy + fees) in satoshis
    pub reward: u64,
}

/// Mining pool identification for a block
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockPool {
    /// Unique pool identifier
    pub id: u8,

    /// Pool name
    pub name: String,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,
}
