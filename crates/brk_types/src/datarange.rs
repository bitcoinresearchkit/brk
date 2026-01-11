use schemars::JsonSchema;
use serde::Deserialize;

use crate::{de_unquote_i64, de_unquote_usize};

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

    /// Number of values to return (ignored if `end` is set)
    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    #[schemars(example = 1, example = 10, example = 100)]
    count: Option<usize>,
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

    pub fn set_count(mut self, count: usize) -> Self {
        self.count.replace(count);
        self
    }

    /// Get the raw `start` value
    pub fn start(&self) -> Option<i64> {
        self.start
    }

    /// Get `end` value, computing it from `start + count` if `end` is unset but `count` is set.
    /// Requires the vec length to resolve negative `start` indices before adding count.
    pub fn end_for_len(&self, len: usize) -> Option<i64> {
        if self.end.is_some() {
            return self.end;
        }

        self.count.map(|count| {
            let resolved_start = self.resolve_index(self.start, len, 0);
            (resolved_start + count).min(len) as i64
        })
    }

    /// Returns a string for etag/cache key generation that captures all range parameters
    pub fn etag_suffix(&self) -> String {
        format!("{:?}{:?}{:?}", self.start, self.end, self.count)
    }

    fn resolve_index(&self, idx: Option<i64>, len: usize, default: usize) -> usize {
        match idx {
            None => default,
            Some(i) if i >= 0 => (i as usize).min(len),
            Some(i) => len.saturating_sub((-i) as usize),
        }
    }
}
