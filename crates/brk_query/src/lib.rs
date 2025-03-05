#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use brk_computer::Computer;
use brk_indexer::Indexer;
use tabled::settings::Style;

mod format;
mod index;
mod output;
mod params;
mod table;
mod tree;

pub use format::Format;
pub use index::Index;
pub use output::{Output, Value};
pub use params::Params;
pub use table::Tabled;
use tree::VecIdToIndexToVec;

pub struct Query<'a> {
    pub vecid_to_index_to_vec: VecIdToIndexToVec<'a>,
    indexer: &'a Indexer,
    computer: &'a Computer,
}

impl<'a> Query<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut vecs = VecIdToIndexToVec::default();

        indexer
            .vecs()
            .as_any_vecs()
            .into_iter()
            .for_each(|vec| vecs.insert(vec));

        computer
            .vecs()
            .as_any_vecs()
            .into_iter()
            .for_each(|vec| vecs.insert(vec));

        Self {
            vecid_to_index_to_vec: vecs,
            indexer,
            computer,
        }
    }

    pub fn search(
        &self,
        index: Index,
        ids: &[&str],
        from: Option<i64>,
        to: Option<i64>,
        format: Option<Format>,
    ) -> color_eyre::Result<Output> {
        let tuples = ids
            .iter()
            .map(|s| {
                let mut id = s.to_lowercase().replace("_", "-");
                let mut res = self.vecid_to_index_to_vec.get(&id);
                if res.is_none() {
                    if let Ok(index) = Index::try_from(id.as_str()) {
                        id = index.possible_values().last().unwrap().to_string();
                        res = self.vecid_to_index_to_vec.get(&id)
                    }
                }
                (id, res)
            })
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, vec)| (id, vec.unwrap()))
            .collect::<Vec<_>>();

        if tuples.is_empty() {
            return Ok(Output::default(format));
        }

        let mut values = tuples
            .iter()
            .flat_map(|(_, i_to_v)| i_to_v.get(&index))
            .map(|vec| -> brk_vec::Result<Vec<serde_json::Value>> {
                vec.collect_range_values(from, to)
            })
            .collect::<brk_vec::Result<Vec<_>>>()?;

        if values.is_empty() {
            return Ok(Output::default(format));
        }

        let ids_last_i = tuples.len() - 1;

        Ok(match format {
            Some(Format::CSV) | Some(Format::TSV) => {
                let delimiter = if format == Some(Format::CSV) {
                    ','
                } else {
                    '\t'
                };

                let mut text = tuples
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
                    Output::CSV(text)
                } else {
                    Output::TSV(text)
                }
            }
            Some(Format::MD) => {
                let mut table =
                    values.to_table(tuples.iter().map(|(s, _)| s.to_owned()).collect::<Vec<_>>());

                table.with(Style::markdown());

                Output::MD(table.to_string())
            }
            Some(Format::JSON) | None => {
                if values.len() == 1 {
                    let mut values = values.pop().unwrap();
                    if values.len() == 1 {
                        let value = values.pop().unwrap();
                        Output::Json(Value::Single(value))
                    } else {
                        Output::Json(Value::List(values))
                    }
                } else {
                    Output::Json(Value::Matrix(values))
                }
            }
        })
    }
}
