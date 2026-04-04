use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Pool, PoolSlug, Sats};

/// Detailed pool information with statistics across time periods
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

    /// Total reward earned by this pool (sats, all time; None for minor pools)
    #[serde(rename = "totalReward")]
    pub total_reward: Option<Sats>,
}

/// Pool information for detail view
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolDetailInfo {
    /// Pool identifier
    pub id: u8,

    /// Pool name
    pub name: Cow<'static, str>,

    /// Pool website URL
    pub link: Cow<'static, str>,

    /// Known payout addresses
    pub addresses: Vec<Cow<'static, str>>,

    /// Coinbase tag patterns (regexes)
    pub regexes: Vec<Cow<'static, str>>,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,

    /// Unique pool identifier
    pub unique_id: u8,
}

impl From<&'static Pool> for PoolDetailInfo {
    fn from(pool: &'static Pool) -> Self {
        Self {
            id: pool.mempool_id(),
            name: Cow::Borrowed(pool.name),
            link: Cow::Borrowed(pool.link),
            addresses: pool.addrs.iter().map(|&s| Cow::Borrowed(s)).collect(),
            regexes: pool.tags.iter().map(|&s| Cow::Borrowed(s)).collect(),
            slug: pool.slug(),
            unique_id: pool.mempool_unique_id(),
        }
    }
}

/// Block counts for different time periods
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolBlockCounts {
    /// Total blocks mined (all time)
    pub all: u64,

    /// Blocks mined in last 24 hours
    #[serde(rename = "24h")]
    pub day: u64,

    /// Blocks mined in last week
    #[serde(rename = "1w")]
    pub week: u64,
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
