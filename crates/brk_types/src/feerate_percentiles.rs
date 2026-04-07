use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::FeeRate;

/// Fee rate percentiles (min, 10%, 25%, 50%, 75%, 90%, max).
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct FeeRatePercentiles {
    /// Minimum fee rate (sat/vB)
    #[serde(rename = "avgFee_0")]
    pub min: FeeRate,
    /// 10th percentile fee rate (sat/vB)
    #[serde(rename = "avgFee_10")]
    pub pct10: FeeRate,
    /// 25th percentile fee rate (sat/vB)
    #[serde(rename = "avgFee_25")]
    pub pct25: FeeRate,
    /// Median fee rate (sat/vB)
    #[serde(rename = "avgFee_50")]
    pub median: FeeRate,
    /// 75th percentile fee rate (sat/vB)
    #[serde(rename = "avgFee_75")]
    pub pct75: FeeRate,
    /// 90th percentile fee rate (sat/vB)
    #[serde(rename = "avgFee_90")]
    pub pct90: FeeRate,
    /// Maximum fee rate (sat/vB)
    #[serde(rename = "avgFee_100")]
    pub max: FeeRate,
}

impl FeeRatePercentiles {
    pub fn new(
        min: FeeRate,
        pct10: FeeRate,
        pct25: FeeRate,
        median: FeeRate,
        pct75: FeeRate,
        pct90: FeeRate,
        max: FeeRate,
    ) -> Self {
        Self {
            min,
            pct10,
            pct25,
            median,
            pct75,
            pct90,
            max,
        }
    }

    /// Convert to array format [min, 10%, 25%, 50%, 75%, 90%, max].
    pub fn to_array(&self) -> [FeeRate; 7] {
        [
            self.min,
            self.pct10,
            self.pct25,
            self.median,
            self.pct75,
            self.pct90,
            self.max,
        ]
    }

    /// Create from array format [min, 10%, 25%, 50%, 75%, 90%, max].
    pub fn from_array(arr: [FeeRate; 7]) -> Self {
        Self {
            min: arr[0],
            pct10: arr[1],
            pct25: arr[2],
            median: arr[3],
            pct75: arr[4],
            pct90: arr[5],
            max: arr[6],
        }
    }
}
