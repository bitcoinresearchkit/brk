use std::ops::Deref;

use brk_structs::Index;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    Format,
    deser::{de_unquote_i64, de_unquote_usize},
    metrics::MaybeMetrics,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Params {
    /// Requested metrics
    #[serde(alias = "m")]
    pub metrics: MaybeMetrics,

    /// Requested index
    #[serde(alias = "i")]
    pub index: Index,

    #[serde(flatten)]
    pub rest: ParamsOpt,
}

impl Deref for Params {
    type Target = ParamsOpt;
    fn deref(&self) -> &Self::Target {
        &self.rest
    }
}

impl From<((Index, String), ParamsOpt)> for Params {
    fn from(((index, metric), rest): ((Index, String), ParamsOpt)) -> Self {
        Self {
            index,
            metrics: MaybeMetrics::from(metric),
            rest,
        }
    }
}

#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct ParamsOpt {
    /// Inclusive starting index, if negative will be from the end
    #[serde(default, alias = "f", deserialize_with = "de_unquote_i64")]
    from: Option<i64>,

    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    to: Option<i64>,

    /// Number of values requested
    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    count: Option<usize>,

    /// Format of the output
    #[serde(default)]
    format: Format,
}

impl ParamsOpt {
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

    pub fn format(&self) -> Format {
        self.format
    }
}

#[derive(Debug, Deserialize)]
pub struct ParamsDeprec {
    #[serde(alias = "i")]
    pub index: Index,
    #[serde(alias = "v")]
    pub ids: MaybeMetrics,
    #[serde(flatten)]
    pub rest: ParamsOpt,
}

impl From<ParamsDeprec> for Params {
    fn from(value: ParamsDeprec) -> Self {
        Params {
            index: value.index,
            metrics: value.ids,
            rest: value.rest,
        }
    }
}
