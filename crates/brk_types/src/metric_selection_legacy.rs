use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRangeFormat, Index, MetricSelection, Metrics};

/// Legacy metric selection parameters (deprecated)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetricSelectionLegacy {
    #[serde(alias = "i")]
    pub index: Index,
    #[serde(alias = "v")]
    pub ids: Metrics,
    #[serde(flatten)]
    pub range: DataRangeFormat,
}

impl From<MetricSelectionLegacy> for MetricSelection {
    #[inline]
    fn from(value: MetricSelectionLegacy) -> Self {
        MetricSelection {
            index: value.index,
            metrics: value.ids,
            range: value.range,
        }
    }
}
