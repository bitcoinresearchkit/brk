use schemars::JsonSchema;
use serde::Serialize;

use crate::{Height, Timestamp};

use super::FeeRatePercentiles;

/// A single block fee rates data point with percentiles.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockFeeRatesEntry {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    #[serde(flatten)]
    pub percentiles: FeeRatePercentiles,
}
