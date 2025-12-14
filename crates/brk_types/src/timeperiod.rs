use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Time period for mining statistics.
///
/// Used to specify the lookback window for pool statistics, hashrate calculations,
/// and other time-based mining metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TimePeriod {
    /// Last 24 hours (~144 blocks)
    #[serde(rename = "24h")]
    Day,
    /// Last 3 days (~432 blocks)
    #[serde(rename = "3d")]
    ThreeDays,
    /// Last week (~1008 blocks)
    #[serde(rename = "1w")]
    Week,
    /// Last month (~4320 blocks)
    #[serde(rename = "1m")]
    Month,
    /// Last 3 months (~12960 blocks)
    #[serde(rename = "3m")]
    ThreeMonths,
    /// Last 6 months (~25920 blocks)
    #[serde(rename = "6m")]
    SixMonths,
    /// Last year (~52560 blocks)
    #[serde(rename = "1y")]
    Year,
    /// Last 2 years (~105120 blocks)
    #[serde(rename = "2y")]
    TwoYears,
    /// Last 3 years (~157680 blocks)
    #[serde(rename = "3y")]
    ThreeYears,
}

impl TimePeriod {
    /// Approximate number of blocks for this time period (10 min per block average)
    pub fn block_count(&self) -> usize {
        match self {
            TimePeriod::Day => 144,
            TimePeriod::ThreeDays => 432,
            TimePeriod::Week => 1008,
            TimePeriod::Month => 4320,
            TimePeriod::ThreeMonths => 12960,
            TimePeriod::SixMonths => 25920,
            TimePeriod::Year => 52560,
            TimePeriod::TwoYears => 105120,
            TimePeriod::ThreeYears => 157680,
        }
    }

    /// Parse from URL path segment
    pub fn from_path(s: &str) -> Option<Self> {
        match s {
            "24h" => Some(TimePeriod::Day),
            "3d" => Some(TimePeriod::ThreeDays),
            "1w" => Some(TimePeriod::Week),
            "1m" => Some(TimePeriod::Month),
            "3m" => Some(TimePeriod::ThreeMonths),
            "6m" => Some(TimePeriod::SixMonths),
            "1y" => Some(TimePeriod::Year),
            "2y" => Some(TimePeriod::TwoYears),
            "3y" => Some(TimePeriod::ThreeYears),
            _ => None,
        }
    }
}
