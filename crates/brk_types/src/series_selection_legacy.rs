use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRangeFormat, Index, SeriesList, SeriesSelection};

/// Legacy series selection parameters (deprecated)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SeriesSelectionLegacy {
    #[serde(alias = "i")]
    pub index: Index,
    #[serde(alias = "v")]
    pub ids: SeriesList,
    #[serde(flatten)]
    pub range: DataRangeFormat,
}

impl From<SeriesSelectionLegacy> for SeriesSelection {
    #[inline]
    fn from(value: SeriesSelectionLegacy) -> Self {
        SeriesSelection {
            index: value.index,
            series: value.ids,
            range: value.range,
        }
    }
}
