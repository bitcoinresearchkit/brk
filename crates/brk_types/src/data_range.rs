use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Limit, RangeIndex, de_unquote_limit};

/// Range parameters for slicing data
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRange {
    /// Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
    #[serde(default, alias = "s", alias = "from", alias = "f")]
    start: Option<RangeIndex>,

    /// Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
    #[serde(default, alias = "e", alias = "to", alias = "t")]
    end: Option<RangeIndex>,

    /// Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
    #[serde(
        default,
        alias = "l",
        alias = "count",
        alias = "c",
        deserialize_with = "de_unquote_limit"
    )]
    limit: Option<Limit>,
}

impl DataRange {
    pub fn set_start(mut self, start: i64) -> Self {
        self.start.replace(RangeIndex::Int(start));
        self
    }

    pub fn set_end(mut self, end: i64) -> Self {
        self.end.replace(RangeIndex::Int(end));
        self
    }

    pub fn set_limit(mut self, limit: Limit) -> Self {
        self.limit.replace(limit);
        self
    }

    pub fn start(&self) -> Option<RangeIndex> {
        self.start
    }

    pub fn end(&self) -> Option<RangeIndex> {
        self.end
    }

    pub fn limit(&self) -> Option<Limit> {
        self.limit
    }

}
