use std::ops::Deref;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRangeFormat, Index, Metric, Metrics};

/// Selection of metrics to query
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetricSelection {
    /// Requested metrics
    #[serde(alias = "m")]
    pub metrics: Metrics,

    /// Index to query
    #[serde(alias = "i")]
    pub index: Index,

    #[serde(flatten)]
    pub range: DataRangeFormat,
}

impl Deref for MetricSelection {
    type Target = DataRangeFormat;
    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl From<(Index, Metric, DataRangeFormat)> for MetricSelection {
    #[inline]
    fn from((index, metric, range): (Index, Metric, DataRangeFormat)) -> Self {
        Self {
            index,
            metrics: Metrics::from(metric),
            range,
        }
    }
}

impl From<(Index, Metrics, DataRangeFormat)> for MetricSelection {
    #[inline]
    fn from((index, metrics, range): (Index, Metrics, DataRangeFormat)) -> Self {
        Self {
            index,
            metrics,
            range,
        }
    }
}
