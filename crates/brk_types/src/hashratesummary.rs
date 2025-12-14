use schemars::JsonSchema;
use serde::Serialize;

use super::{DifficultyEntry, HashrateEntry};

/// Summary of network hashrate and difficulty data.
#[derive(Debug, Serialize, JsonSchema)]
pub struct HashrateSummary {
    /// Historical hashrate data points.
    pub hashrates: Vec<HashrateEntry>,
    /// Historical difficulty adjustments.
    pub difficulty: Vec<DifficultyEntry>,
    /// Current network hashrate (H/s).
    #[serde(rename = "currentHashrate")]
    pub current_hashrate: u128,
    /// Current network difficulty.
    #[serde(rename = "currentDifficulty")]
    pub current_difficulty: f64,
}
