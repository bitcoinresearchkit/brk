use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Index, Metric};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct MetricWithIndex {
    /// Metric name
    pub metric: Metric,

    /// Aggregation index
    pub index: Index,
}

impl MetricWithIndex {
    pub fn new(metric: impl Into<Metric>, index: Index) -> Self {
        Self {
            metric: metric.into(),
            index,
        }
    }
}

impl From<(Metric, Index)> for MetricWithIndex {
    fn from((metric, index): (Metric, Index)) -> Self {
        Self { metric, index }
    }
}

impl From<(&str, Index)> for MetricWithIndex {
    fn from((metric, index): (&str, Index)) -> Self {
        Self {
            metric: metric.into(),
            index,
        }
    }
}
