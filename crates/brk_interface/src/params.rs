use std::ops::Deref;

use rmcp::schemars::{self, JsonSchema};
use serde::Deserialize;

use crate::{
    Format, Index,
    deser::{de_unquote_i64, de_unquote_usize},
    maybe_ids::MaybeIds,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Params {
    #[serde(alias = "i")]
    #[schemars(description = "Index of requested vecs")]
    pub index: Index,

    #[serde(alias = "v")]
    #[schemars(description = "Ids of requested vecs")]
    pub ids: MaybeIds,

    #[serde(flatten)]
    pub rest: ParamsOpt,
}
serde_with::flattened_maybe!(deserialize_rest, "rest");

impl Deref for Params {
    type Target = ParamsOpt;
    fn deref(&self) -> &Self::Target {
        &self.rest
    }
}

impl From<((Index, String), ParamsOpt)> for Params {
    fn from(((index, id), rest): ((Index, String), ParamsOpt)) -> Self {
        Self {
            index,
            ids: MaybeIds::from(id),
            rest,
        }
    }
}

#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct ParamsOpt {
    #[serde(default, alias = "f", deserialize_with = "de_unquote_i64")]
    /// Inclusive starting index, if negative will be from the end
    #[schemars(description = "Inclusive starting index, if negative will be from the end")]
    from: Option<i64>,

    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    #[schemars(
        description = "Exclusive ending index, if negative will be from the end, overrides 'count'"
    )]
    to: Option<i64>,

    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    /// Number of values requested
    #[schemars(description = "Number of values requested")]
    count: Option<usize>,

    #[serde(default)]
    /// Format of the output
    #[schemars(description = "Format of the output")]
    format: Option<Format>,
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

    pub fn set_format(mut self, format: Format) -> Self {
        self.format.replace(format);
        self
    }

    pub fn from(&self) -> Option<i64> {
        self.from
    }

    pub fn to(&self) -> Option<i64> {
        if self.to.is_none() {
            if let Some(c) = self.count {
                let c = c as i64;
                if let Some(f) = self.from {
                    if f >= 0 || f.abs() > c {
                        return Some(f + c);
                    }
                } else {
                    return Some(c);
                }
            }
        }
        self.to
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IdParam {
    pub id: String,
}
