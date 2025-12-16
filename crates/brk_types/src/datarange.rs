use schemars::JsonSchema;
use serde::Deserialize;

use crate::{de_unquote_i64, de_unquote_usize};

/// Range parameters for slicing data
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRange {
    /// Inclusive starting index, if negative will be from the end
    #[serde(default, alias = "f", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 0, example = -1, example = -10, example = -1000)]
    from: Option<i64>,

    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    #[schemars(example = 1000)]
    to: Option<i64>,

    /// Number of values requested
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

    pub fn from(&self) -> Option<i64> {
        self.from
    }

    pub fn to(&self) -> Option<i64> {
        if self.to.is_none()
            && let Some(c) = self.count
        {
            let c = c as i64;
            if let Some(f) = self.from {
                if f >= 0 || f.abs() > c {
                    return Some(f + c);
                }
            } else {
                return Some(c);
            }
        }
        self.to
    }
}
