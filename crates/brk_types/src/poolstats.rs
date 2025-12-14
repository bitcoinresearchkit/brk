use schemars::JsonSchema;
use serde::Serialize;

use crate::{Pool, PoolSlug};

/// Mining pool with block statistics for a time period
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolStats {
    /// Unique pool identifier
    #[serde(rename = "poolId")]
    pub pool_id: u8,

    /// Pool name
    pub name: &'static str,

    /// Pool website URL
    pub link: &'static str,

    /// Number of blocks mined in the time period
    #[serde(rename = "blockCount")]
    pub block_count: u32,

    /// Pool ranking by block count (1 = most blocks)
    pub rank: u32,

    /// Number of empty blocks mined
    #[serde(rename = "emptyBlocks")]
    pub empty_blocks: u32,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,

    /// Pool's share of total blocks (0.0 - 1.0)
    pub share: f64,
}

impl PoolStats {
    /// Create a new PoolStats from a Pool reference
    pub fn new(pool: &'static Pool, block_count: u32, rank: u32, share: f64) -> Self {
        Self {
            pool_id: pool.unique_id(),
            name: pool.name,
            link: pool.link,
            block_count,
            rank,
            empty_blocks: 0, // TODO: track empty blocks if needed
            slug: pool.slug(),
            share,
        }
    }
}
