use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Index, Series};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SeriesWithIndex {
    /// Series name
    pub series: Series,

    /// Aggregation index
    pub index: Index,
}

impl SeriesWithIndex {
    pub fn new(series: impl Into<Series>, index: Index) -> Self {
        Self {
            series: series.into(),
            index,
        }
    }
}

impl From<(Series, Index)> for SeriesWithIndex {
    fn from((series, index): (Series, Index)) -> Self {
        Self { series, index }
    }
}

impl From<(&str, Index)> for SeriesWithIndex {
    fn from((series, index): (&str, Index)) -> Self {
        Self {
            series: series.into(),
            index,
        }
    }
}
