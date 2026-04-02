use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Height, Timestamp};

/// A single difficulty data point in the hashrate summary.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DifficultyEntry {
    /// Unix timestamp of the difficulty adjustment
    pub time: Timestamp,
    /// Block height of the adjustment
    pub height: Height,
    /// Difficulty value
    pub difficulty: f64,
    /// Adjustment ratio (new/previous, e.g. 1.068 = +6.8%)
    pub adjustment: f64,
}
