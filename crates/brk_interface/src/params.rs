use std::ops::Deref;

use brk_rmcp::schemars::{self, JsonSchema};
use serde::{Deserialize, Deserializer};

use crate::{Format, Index, maybe_ids::MaybeIds};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Params {
    #[serde(alias = "i")]
    #[schemars(description = "Index of the values requested")]
    pub index: Index,

    #[serde(alias = "v")]
    #[schemars(description = "Ids of the requested vecs")]
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
    from: Option<i64>,

    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    to: Option<i64>,

    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    /// Number of values
    count: Option<usize>,

    /// Format of the output
    #[serde(default)]
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

fn de_unquote_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(serde_json::Value::String(mut s)) => {
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Number(n)) => {
            // If it's a number, convert it to i64
            n.as_i64()
                .ok_or_else(|| serde::de::Error::custom("number out of range"))
                .map(Some)
        }
        _ => Err(serde::de::Error::custom("expected a string or number")),
    }
}

fn de_unquote_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(serde_json::Value::String(mut s)) => {
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            s.parse::<usize>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Number(n)) => {
            // If it's a number, convert it to usize
            n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("number out of range"))
                .map(|v| v as usize)
                .map(Some)
        }
        _ => {
            dbg!(value);
            Err(serde::de::Error::custom("expected a string or number"))
        }
    }
}
