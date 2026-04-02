use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Height, Timestamp, Weight};

/// A single block weight data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockWeightEntry {
    /// Average block height in this window
    #[schemars(example = 890621)]
    pub avg_height: Height,
    /// Unix timestamp at the window midpoint
    #[schemars(example = 1743631892)]
    pub timestamp: Timestamp,
    /// Rolling 24h median block weight (weight units)
    #[schemars(example = 3990000)]
    pub avg_weight: Weight,
}
