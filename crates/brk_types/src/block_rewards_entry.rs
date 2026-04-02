use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Dollars, Height, Sats, Timestamp};

/// A single block rewards data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockRewardsEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_rewards: Sats,
    /// BTC/USD price at that height
    #[serde(rename = "USD")]
    pub usd: Dollars,
}
