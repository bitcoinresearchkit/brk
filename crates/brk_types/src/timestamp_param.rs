use schemars::JsonSchema;
use serde::Deserialize;

use crate::Timestamp;

#[derive(Deserialize, JsonSchema)]
pub struct TimestampParam {
    pub timestamp: Timestamp,
}

#[derive(Deserialize, JsonSchema)]
pub struct OptionalTimestampParam {
    pub timestamp: Option<Timestamp>,
}
