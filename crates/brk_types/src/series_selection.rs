use derive_more::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRangeFormat, Index, Series, SeriesList};

/// Selection of series to query
#[derive(Debug, Deref, Deserialize, JsonSchema)]
pub struct SeriesSelection {
    /// Requested series
    #[serde(alias = "m", alias = "metrics")]
    pub series: SeriesList,

    /// Index to query
    #[serde(alias = "i")]
    pub index: Index,

    #[deref]
    #[serde(flatten)]
    pub range: DataRangeFormat,
}

impl From<(Index, Series, DataRangeFormat)> for SeriesSelection {
    #[inline]
    fn from((index, series, range): (Index, Series, DataRangeFormat)) -> Self {
        Self {
            index,
            series: SeriesList::from(series),
            range,
        }
    }
}

impl From<(Index, SeriesList, DataRangeFormat)> for SeriesSelection {
    #[inline]
    fn from((index, series, range): (Index, SeriesList, DataRangeFormat)) -> Self {
        Self {
            index,
            series,
            range,
        }
    }
}
