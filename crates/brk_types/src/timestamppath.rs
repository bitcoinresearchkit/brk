use schemars::JsonSchema;
use serde::Deserialize;

use crate::Timestamp;

#[derive(Deserialize, JsonSchema)]
pub struct TimestampPath {
    /// UNIX timestamp in seconds
    #[schemars(example = 1672531200)]
    pub timestamp: Timestamp,
}
