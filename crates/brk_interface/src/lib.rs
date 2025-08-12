#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::Height;
use tabled::settings::Style;
use vecdb::{AnyCollectableVec, AnyStoredVec};

mod deser;
mod format;
mod index;
mod maybe_ids;
mod output;
mod pagination;
mod params;
mod table;
mod vecs;

pub use format::Format;
pub use index::Index;
pub use output::{Output, Value};
pub use pagination::{PaginatedIndexParam, PaginationParam};
pub use params::{IdParam, Params, ParamsOpt};
pub use table::Tabled;
use vecs::Vecs;

use crate::vecs::{IdToVec, IndexToVec};

#[allow(dead_code)]
pub struct Interface<'a> {
    vecs: Vecs<'a>,
    indexer: &'a Indexer,
    computer: &'a Computer,
}

impl<'a> Interface<'a> {
    pub fn build(indexer: &Indexer, computer: &Computer) -> Self {
        let indexer = indexer.static_clone();
        let computer = computer.static_clone();
        let vecs = Vecs::build(indexer, computer);

        Self {
            vecs,
            indexer,
            computer,
        }
    }

    pub fn get_height(&self) -> Height {
        Height::from(self.indexer.vecs.height_to_blockhash.stamp())
    }

    pub fn search(&self, params: &Params) -> Vec<(String, &&dyn AnyCollectableVec)> {
        let tuples = params
            .ids
            .iter()
            .flat_map(|s| {
                s.to_lowercase()
                    .replace("-", "_")
                    .split_whitespace()
                    .flat_map(|s| {
                        s.split(',')
                            .flat_map(|s| s.split('+').map(|s| s.to_string()))
                    })
                    .collect::<Vec<_>>()
            })
            .map(|mut id| {
                let mut res = self.vecs.id_to_index_to_vec.get(id.as_str());
                if res.is_none()
                    && let Ok(index) = Index::try_from(id.as_str())
                {
                    id = index.possible_values().last().unwrap().to_string();
                    res = self.vecs.id_to_index_to_vec.get(id.as_str())
                }
                (id, res)
            })
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, vec)| (id, vec.unwrap()))
            .collect::<Vec<_>>();

        tuples
            .iter()
            .flat_map(|(str, i_to_v)| i_to_v.get(&params.index).map(|vec| (str.to_owned(), vec)))
            .collect::<Vec<_>>()
    }

    pub fn format(
        &self,
        vecs: Vec<(String, &&dyn AnyCollectableVec)>,
        params: &ParamsOpt,
    ) -> Result<Output> {
        let from = params.from().map(|from| {
            vecs.iter()
                .map(|(_, v)| v.i64_to_usize(from))
                .min()
                .unwrap_or_default()
        });

        let to = params.to().map(|to| {
            vecs.iter()
                .map(|(_, v)| v.i64_to_usize(to))
                .min()
                .unwrap_or_default()
        });

        let mut values = vecs
            .iter()
            .map(|(_, vec)| -> Result<Vec<serde_json::Value>> {
                Ok(vec.collect_range_serde_json(from, to)?)
            })
            .collect::<Result<Vec<_>>>()?;

        let format = params.format();

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

    pub fn search_and_format(&self, params: Params) -> Result<Output> {
        self.format(self.search(&params), &params.rest)
    }

    pub fn id_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        &self.vecs.id_to_index_to_vec
    }

    pub fn index_to_id_to_vec(&self) -> &BTreeMap<Index, IdToVec<'_>> {
        &self.vecs.index_to_id_to_vec
    }

    pub fn get_vecid_count(&self) -> usize {
        self.vecs.id_count
    }

    pub fn get_index_count(&self) -> usize {
        self.vecs.index_count
    }

    pub fn get_vec_count(&self) -> usize {
        self.vecs.vec_count
    }

    pub fn get_indexes(&self) -> &[&'static str] {
        &self.vecs.indexes
    }

    pub fn get_accepted_indexes(&self) -> &BTreeMap<&'static str, &'static [&'static str]> {
        &self.vecs.accepted_indexes
    }

    pub fn get_vecids(&self, pagination: PaginationParam) -> &[&str] {
        self.vecs.ids(pagination)
    }

    pub fn get_index_to_vecids(&self, paginated_index: PaginatedIndexParam) -> Vec<&str> {
        self.vecs.index_to_ids(paginated_index)
    }

    pub fn get_vecid_to_indexes(&self, id: String) -> Option<&Vec<&'static str>> {
        self.vecs.id_to_indexes(id)
    }
}
