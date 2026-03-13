use schemars::JsonSchema;
use serde::Deserialize;

use super::TimePeriod;

#[derive(Deserialize, JsonSchema)]
pub struct TimePeriodParam {
    #[schemars(example = &"24h")]
    pub time_period: TimePeriod,
}
