use schemars::JsonSchema;
use serde::Serialize;

use super::FeeRatePercentiles;

/// A single block fee rates data point with percentiles.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeeRatesEntry {
    pub avg_height: u32,
    pub timestamp: u32,
    #[serde(flatten)]
    pub percentiles: FeeRatePercentiles,
}
