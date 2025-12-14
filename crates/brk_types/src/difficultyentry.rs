use schemars::JsonSchema;
use serde::Serialize;

use super::{Height, Timestamp};

/// A single difficulty data point.
#[derive(Debug, Serialize, JsonSchema)]
pub struct DifficultyEntry {
    /// Unix timestamp of the difficulty adjustment.
    pub timestamp: Timestamp,
    /// Difficulty value.
    pub difficulty: f64,
    /// Block height of the adjustment.
    pub height: Height,
}
