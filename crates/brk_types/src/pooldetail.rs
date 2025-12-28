use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Pool, PoolSlug};

/// Detailed pool information with statistics across time periods
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolDetail {
    /// Pool information
    pub pool: PoolDetailInfo,

    /// Block counts for different time periods
    #[serde(rename = "blockCount")]
    pub block_count: PoolBlockCounts,

    /// Pool's share of total blocks for different time periods
    #[serde(rename = "blockShare")]
    pub block_share: PoolBlockShares,

    /// Estimated hashrate based on blocks mined
    #[serde(rename = "estimatedHashrate")]
    pub estimated_hashrate: u128,

    /// Self-reported hashrate (if available)
    #[serde(rename = "reportedHashrate")]
    pub reported_hashrate: Option<u128>,
}

/// Pool information for detail view
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolDetailInfo {
    /// Unique pool identifier
    pub id: u8,

    /// Pool name
    pub name: &'static str,

    /// Pool website URL
    pub link: &'static str,

    /// Known payout addresses
    pub addresses: Vec<&'static str>,

    /// Coinbase tag patterns (regexes)
    pub regexes: Vec<&'static str>,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,
}

impl From<&'static Pool> for PoolDetailInfo {
    fn from(pool: &'static Pool) -> Self {
        Self {
            id: pool.unique_id(),
            name: pool.name,
            link: pool.link,
            addresses: pool.addresses.to_vec(),
            regexes: pool.tags.to_vec(),
            slug: pool.slug(),
        }
    }
}

/// Block counts for different time periods
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolBlockCounts {
    /// Total blocks mined (all time)
    pub all: u32,

    /// Blocks mined in last 24 hours
    #[serde(rename = "24h")]
    pub day: u32,

    /// Blocks mined in last week
    #[serde(rename = "1w")]
    pub week: u32,
}

/// Pool's share of total blocks for different time periods
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolBlockShares {
    /// Share of all blocks (0.0 - 1.0)
    pub all: f64,

    /// Share of blocks in last 24 hours
    #[serde(rename = "24h")]
    pub day: f64,

    /// Share of blocks in last week
    #[serde(rename = "1w")]
    pub week: f64,
}
