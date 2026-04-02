use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Height, Timestamp, Weight};

/// A single block weight data point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockWeightEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_weight: Weight,
}
