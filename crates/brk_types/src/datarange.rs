use schemars::JsonSchema;
use serde::Deserialize;

use crate::{de_unquote_i64, de_unquote_limit, Limit};

/// Range parameters for slicing data
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRange {
    /// Inclusive starting index, if negative counts from end
    #[serde(default, alias = "s", alias = "from", alias = "f", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 0, example = -1, example = -10, example = -1000)]
    start: Option<i64>,

    /// Exclusive ending index, if negative counts from end
    #[serde(default, alias = "e", alias = "to", alias = "t", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 1000)]
    end: Option<i64>,

    /// Maximum number of values to return (ignored if `end` is set)
    #[serde(default, alias = "l", alias = "count", alias = "c", deserialize_with = "de_unquote_limit")]
    limit: Option<Limit>,
}

impl DataRange {
    pub fn set_start(mut self, start: i64) -> Self {
        self.start.replace(start);
        self
    }

    pub fn set_end(mut self, end: i64) -> Self {
        self.end.replace(end);
        self
    }

    pub fn set_limit(mut self, limit: Limit) -> Self {
        self.limit.replace(limit);
        self
    }

    /// Get the raw `start` value
    pub fn start(&self) -> Option<i64> {
        self.start
    }

    /// Get `end` value, computing it from `start + limit` if `end` is unset but `limit` is set.
    /// Requires the vec length to resolve negative `start` indices before adding limit.
    pub fn end_for_len(&self, len: usize) -> Option<i64> {
        if self.end.is_some() {
            return self.end;
        }

        self.limit.map(|limit| {
            let resolved_start = self.resolve_index(self.start, len, 0);
            (resolved_start + *limit).min(len) as i64
        })
    }

    fn resolve_index(&self, idx: Option<i64>, len: usize, default: usize) -> usize {
        match idx {
            None => default,
            Some(i) if i >= 0 => (i as usize).min(len),
            Some(i) => len.saturating_sub((-i) as usize),
        }
    }
}
