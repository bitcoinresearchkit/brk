use schemars::JsonSchema;
use serde::Serialize;

use crate::{Height, Sats, Timestamp};

/// A single block fees data point.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeesEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_fees: Sats,
}
