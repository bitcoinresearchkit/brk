use derive_more::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRangeFormat, Index, Metric, Metrics};

/// Selection of metrics to query
#[derive(Debug, Deref, Deserialize, JsonSchema)]
pub struct MetricSelection {
    /// Requested metrics
    #[serde(alias = "m")]
    pub metrics: Metrics,

    /// Index to query
    #[serde(alias = "i")]
    pub index: Index,

    #[deref]
    #[serde(flatten)]
    pub range: DataRangeFormat,
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
