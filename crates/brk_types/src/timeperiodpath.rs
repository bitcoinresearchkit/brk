use schemars::JsonSchema;
use serde::Deserialize;

use super::TimePeriod;

/// Path parameter for mining pool statistics time period
#[derive(Deserialize, JsonSchema)]
pub struct TimePeriodPath {
    /// Time period for statistics.
    /// Valid values: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub time_period: TimePeriod,
}
