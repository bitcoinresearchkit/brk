use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Height, Sats, Timestamp};

/// A single block fees data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeesEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_fees: Sats,
}
