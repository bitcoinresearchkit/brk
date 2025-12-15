use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Index, Metric};

#[derive(Deserialize, JsonSchema)]
pub struct MetricWithIndex {
    pub metric: Metric,
    pub index: Index,
}
