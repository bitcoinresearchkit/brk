#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use brk_computer::Computer;
use brk_core::Result;
use brk_indexer::Indexer;
use brk_vec::AnyCollectableVec;
use tabled::settings::Style;

mod format;
mod index;
mod output;
mod params;
mod table;
mod vec_trees;

pub use format::Format;
pub use index::Index;
pub use output::{Output, Value};
pub use params::{Params, ParamsOpt};
pub use table::Tabled;
use vec_trees::VecTrees;

pub struct Query<'a> {
    pub vec_trees: VecTrees<'a>,
    _indexer: &'a Indexer,
    _computer: &'a Computer,
}

impl<'a> Query<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut vec_trees = VecTrees::default();

        indexer
            .vecs()
            .vecs()
            .into_iter()
            .for_each(|vec| vec_trees.insert(vec));

        computer
            .vecs()
            .into_iter()
            .for_each(|vec| vec_trees.insert(vec));

        Self {
            vec_trees,
            _indexer: indexer,
            _computer: computer,
        }
    }

    pub fn search(&self, index: Index, ids: &[&str]) -> Vec<(String, &&dyn AnyCollectableVec)> {
        let tuples = ids
            .iter()
            .flat_map(|s| {
                s.to_lowercase()
                    .replace("_", "-")
                    .split_whitespace()
                    .flat_map(|s| {
                        s.split(',')
                            .flat_map(|s| s.split('+').map(|s| s.to_string()))
                    })
                    .collect::<Vec<_>>()
            })
            .map(|mut id| {
                let mut res = self.vec_trees.id_to_index_to_vec.get(&id);
                if res.is_none() {
                    if let Ok(index) = Index::try_from(id.as_str()) {
                        id = index.possible_values().last().unwrap().to_string();
                        res = self.vec_trees.id_to_index_to_vec.get(&id)
                    }
                }
                (id, res)
            })
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, vec)| (id, vec.unwrap()))
            .collect::<Vec<_>>();

        tuples
            .iter()
            .flat_map(|(str, i_to_v)| i_to_v.get(&index).map(|vec| (str.to_owned(), vec)))
            .collect::<Vec<_>>()
    }

    pub fn format(
        &self,
        vecs: Vec<(String, &&dyn AnyCollectableVec)>,
        from: Option<i64>,
        to: Option<i64>,
        format: Option<Format>,
    ) -> color_eyre::Result<Output> {
        let mut values = vecs
            .iter()
            .map(|(_, vec)| -> Result<Vec<serde_json::Value>> {
                vec.collect_range_serde_json(from, to)
            })
            .collect::<Result<Vec<_>>>()?;

        if values.is_empty() {
            return Ok(Output::default(format));
        }

        let ids_last_i = vecs.len() - 1;

        Ok(match format {
            Some(Format::CSV) | Some(Format::TSV) => {
                let delimiter = if format == Some(Format::CSV) {
                    ','
                } else {
                    '\t'
                };

                let mut text = vecs
                    .iter()
                    .map(|(id, _)| id.to_owned())
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
                    values.to_table(vecs.iter().map(|(s, _)| s.to_owned()).collect::<Vec<_>>());

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

    pub fn search_and_format(
        &self,
        index: Index,
        ids: &[&str],
        from: Option<i64>,
        to: Option<i64>,
        format: Option<Format>,
    ) -> color_eyre::Result<Output> {
        self.format(self.search(index, ids), from, to, format)
    }
}
