use schemars::JsonSchema;
use serde::Deserialize;

use super::TimePeriod;

#[derive(Deserialize, JsonSchema)]
pub struct TimePeriodParam {
    pub time_period: TimePeriod,
}
