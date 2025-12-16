use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Index, Metric};

#[derive(Deserialize, JsonSchema)]
pub struct MetricWithIndex {
    /// Metric name
    pub metric: Metric,

    /// Aggregation index
    pub index: Index,
}
