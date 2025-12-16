use schemars::JsonSchema;
use serde::Deserialize;

use crate::{de_unquote_i64, de_unquote_usize};

/// Range parameters for slicing data
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRange {
    /// Inclusive starting index, if negative counts from end
    #[serde(default, alias = "f", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 0, example = -1, example = -10, example = -1000)]
    from: Option<i64>,

    /// Exclusive ending index, if negative counts from end
    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 1000)]
    to: Option<i64>,

    /// Number of values to return (ignored if `to` is set)
    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    #[schemars(example = 1, example = 10, example = 100)]
    count: Option<usize>,
}

impl DataRange {
    pub fn set_from(mut self, from: i64) -> Self {
        self.from.replace(from);
        self
    }

    pub fn set_to(mut self, to: i64) -> Self {
        self.to.replace(to);
        self
    }

    pub fn set_count(mut self, count: usize) -> Self {
        self.count.replace(count);
        self
    }

    /// Get the raw `from` value
    pub fn from(&self) -> Option<i64> {
        self.from
    }

    /// Get `to` value, computing it from `from + count` if `to` is unset but `count` is set.
    /// Requires the vec length to resolve negative `from` indices before adding count.
    pub fn to_for_len(&self, len: usize) -> Option<i64> {
        if self.to.is_some() {
            return self.to;
        }

        self.count.map(|count| {
            let resolved_from = self.resolve_index(self.from, len, 0);
            (resolved_from + count).min(len) as i64
        })
    }

    /// Returns a string for etag/cache key generation that captures all range parameters
    pub fn etag_suffix(&self) -> String {
        format!("{:?}{:?}{:?}", self.from, self.to, self.count)
    }

    fn resolve_index(&self, idx: Option<i64>, len: usize, default: usize) -> usize {
        match idx {
            None => default,
            Some(i) if i >= 0 => (i as usize).min(len),
            Some(i) => len.saturating_sub((-i) as usize),
        }
    }
}
