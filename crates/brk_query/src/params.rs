use std::{fmt::Display, ops::Deref, str::FromStr};

use clap::builder::PossibleValuesParser;
use clap_derive::Parser;
use serde::{Deserialize, Deserializer};
use serde_with::{OneOrMany, formats::PreferOne, serde_as};

use crate::{Format, Index};

#[serde_as]
#[derive(Debug, Deserialize, Parser)]
pub struct Params {
    #[clap(short, long, value_parser = PossibleValuesParser::new(Index::all_possible_values()))]
    #[serde(alias = "i")]
    /// Index of the values requested
    pub index: String,
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    #[serde(alias = "v")]
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    /// Names of the values requested
    pub values: Vec<String>,

    #[clap(flatten)]
    #[serde(flatten)]
    pub rest: ParamsOpt,
}

// The macro creates custom deserialization code.
// You need to specify a function name and the field name of the flattened field.
serde_with::flattened_maybe!(deserialize_rest, "rest");

impl Deref for Params {
    type Target = ParamsOpt;
    fn deref(&self) -> &Self::Target {
        &self.rest
    }
}

impl From<((String, String), ParamsOpt)> for Params {
    fn from(((index, id), rest): ((String, String), ParamsOpt)) -> Self {
        Self {
            index,
            values: vec![id],
            rest,
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Parser)]
pub struct ParamsOpt {
    #[clap(short, long, allow_hyphen_values = true)]
    #[serde(default, alias = "f", deserialize_with = "de_unquote_i64")]
    /// Inclusive starting index, if negative will be from the end
    from: Option<i64>,
    #[clap(short, long, allow_hyphen_values = true)]
    #[serde(default, alias = "t", deserialize_with = "de_unquote_i64")]
    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    to: Option<i64>,
    #[clap(short, long, allow_hyphen_values = true)]
    #[serde(default, alias = "c", deserialize_with = "de_unquote_usize")]
    /// Number of values
    count: Option<usize>,
    #[clap(short = 'F', long)]
    /// Format of the output
    format: Option<Format>,
}

impl ParamsOpt {
    pub fn from(&self) -> Option<i64> {
        self.from
    }

    pub fn to(&self) -> Option<i64> {
        if self.to.is_none() {
            if let Some(c) = self.count {
                let c = c as i64;
                if let Some(f) = self.from {
                    if f.is_positive() || f.abs() > c {
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

fn de_unquote_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    de_unquote(deserializer)
}

fn de_unquote_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    de_unquote(deserializer)
}

fn de_unquote<'de, D, F>(deserializer: D) -> Result<Option<F>, D::Error>
where
    D: Deserializer<'de>,
    F: FromStr + Display,
    <F as std::str::FromStr>::Err: Display,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    let s = match opt {
        None => return Ok(None),
        Some(mut s) => {
            // strip any leading/trailing quotes
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            s
        }
    };
    s.parse::<F>()
        .map(Some)
        .map_err(|e| serde::de::Error::custom(format!("cannot parse `{}` as type: {}", s, e)))
}
