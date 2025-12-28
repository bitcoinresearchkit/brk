use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Height;

/// Difficulty adjustment information.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyAdjustment {
    /// Progress through current difficulty epoch (0-100%)
    #[schemars(example = 44.4)]
    pub progress_percent: f64,

    /// Estimated difficulty change at next retarget (%)
    #[schemars(example = 2.5)]
    pub difficulty_change: f64,

    /// Estimated Unix timestamp of next retarget
    #[schemars(example = 1627762478)]
    pub estimated_retarget_date: u64,

    /// Blocks remaining until retarget
    #[schemars(example = 1121)]
    pub remaining_blocks: u32,

    /// Estimated seconds until retarget
    #[schemars(example = 665977)]
    pub remaining_time: u64,

    /// Previous difficulty adjustment (%)
    #[schemars(example = -4.8)]
    pub previous_retarget: f64,

    /// Height of next retarget
    #[schemars(example = 741888)]
    pub next_retarget_height: Height,

    /// Average block time in current epoch (seconds)
    #[schemars(example = 580)]
    pub time_avg: u64,

    /// Time-adjusted average (accounting for timestamp manipulation)
    #[schemars(example = 580)]
    pub adjusted_time_avg: u64,

    /// Time offset from expected schedule (seconds)
    #[schemars(example = 0)]
    pub time_offset: i64,
}
