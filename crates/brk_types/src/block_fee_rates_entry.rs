use schemars::JsonSchema;
use serde::Serialize;

use crate::{Height, Timestamp};

use super::FeeRatePercentiles;

/// A single block fee rates data point with percentiles.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeeRatesEntry {
    /// Average block height in this window
    pub avg_height: Height,
    /// Unix timestamp at the window midpoint
    pub timestamp: Timestamp,
    /// Fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max)
    #[serde(flatten)]
    pub percentiles: FeeRatePercentiles,
}
