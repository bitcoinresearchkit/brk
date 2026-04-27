use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Index, SeriesList, SeriesSelection, with_range_format::with_range_format};

with_range_format! {
    /// Legacy series selection parameters (deprecated)
    #[derive(Debug, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct SeriesSelectionLegacy {
        #[serde(alias = "i")]
        pub index: Index,
        #[serde(alias = "v")]
        pub ids: SeriesList,
    }
}

impl From<SeriesSelectionLegacy> for SeriesSelection {
    #[inline]
    fn from(value: SeriesSelectionLegacy) -> Self {
        let start = value.start();
        let end = value.end();
        let limit = value.limit();
        let format = value.format();
        SeriesSelection {
            index: value.index,
            series: value.ids,
            start,
            end,
            limit,
            format,
        }
    }
}
