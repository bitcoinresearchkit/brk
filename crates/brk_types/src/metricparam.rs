use schemars::JsonSchema;
use serde::Deserialize;

use crate::Metric;

#[derive(Deserialize, JsonSchema)]
pub struct MetricParam {
    pub metric: Metric,
}
