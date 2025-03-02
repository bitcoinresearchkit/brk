#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("main.rs")]
#![doc = "```"]

mod format;
mod index;
mod params;
mod tree;

use std::fmt;

use brk_computer::Computer;
use brk_indexer::Indexer;
pub use format::Format;
pub use index::Index;
pub use params::Params;
use serde::Serialize;
use tree::VecIdToIndexToVec;

pub struct Query<'a> {
    pub vecid_to_index_to_vec: VecIdToIndexToVec<'a>,
    indexer: &'a Indexer,
    computer: &'a Computer,
}

impl<'a> Query<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut vecs = VecIdToIndexToVec::default();

        indexer.vecs.as_any_vecs().into_iter().for_each(|vec| vecs.insert(vec));
        computer.vecs.as_any_vecs().into_iter().for_each(|vec| vecs.insert(vec));

        Self {
            vecid_to_index_to_vec: vecs,
            indexer,
            computer,
        }
    }

    pub fn search(
        &self,
        index: Index,
        values: &[&str],
        from: Option<i64>,
        to: Option<i64>,
        format: Option<Format>,
    ) -> color_eyre::Result<QueryResponse> {
        let ids = values
            .iter()
            .map(|s| {
                (
                    s.to_owned(),
                    self.vecid_to_index_to_vec.get(&s.to_lowercase().replace("_", "-")),
                )
            })
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, vec)| (id, vec.unwrap()))
            .collect::<Vec<_>>();

        if ids.is_empty() {
            return Ok(QueryResponse::default(format));
        }

        let mut values = ids
            .iter()
            .flat_map(|(_, i_to_v)| i_to_v.get(&index))
            .map(|vec| -> brk_vec::Result<Vec<serde_json::Value>> { vec.collect_range_values(from, to) })
            .collect::<brk_vec::Result<Vec<_>>>()?;

        if values.is_empty() {
            return Ok(QueryResponse::default(format));
        }

        let ids_last_i = ids.len() - 1;

        Ok(match format {
            Some(Format::CSV) | Some(Format::TSV) => {
                let delimiter = if format == Some(Format::CSV) { ',' } else { '\t' };

                let mut text = ids
                    .into_iter()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>()
                    .join(&delimiter.to_string());

                text.push('\n');

                let values_len = values.first().unwrap().len();

                (0..values_len).for_each(|i| {
                    let mut line = "".to_string();
                    values.iter().enumerate().for_each(|(id_i, v)| {
                        line += &v.get(i).unwrap().to_string();
                        if id_i == ids_last_i {
                            line.push('\n');
                        } else {
                            line.push(delimiter);
                        }
                    });
                    text += &line;
                });

                if format == Some(Format::CSV) {
                    QueryResponse::CSV(text)
                } else {
                    QueryResponse::TSV(text)
                }
            }
            Some(Format::JSON) | None => {
                if values.len() == 1 {
                    let mut values = values.pop().unwrap();
                    if values.len() == 1 {
                        let value = values.pop().unwrap();
                        QueryResponse::Json(Value::Single(value))
                    } else {
                        QueryResponse::Json(Value::List(values))
                    }
                } else {
                    QueryResponse::Json(Value::Matrix(values))
                }
            }
        })
    }
}

#[derive(Debug)]
pub enum QueryResponse {
    Json(Value),
    CSV(String),
    TSV(String),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    Matrix(Vec<Vec<serde_json::Value>>),
    List(Vec<serde_json::Value>),
    Single(serde_json::Value),
}

impl QueryResponse {
    fn default(format: Option<Format>) -> Self {
        match format {
            Some(Format::CSV) => QueryResponse::CSV("".to_string()),
            Some(Format::TSV) => QueryResponse::TSV("".to_string()),
            _ => QueryResponse::Json(Value::Single(serde_json::Value::Null)),
        }
    }
}

impl fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(value) => write!(f, "{}", serde_json::to_string_pretty(value).unwrap()),
            Self::CSV(string) => write!(f, "{}", string),
            Self::TSV(string) => write!(f, "{}", string),
        }
    }
}
