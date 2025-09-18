#![doc = include_str!("../README.md")]

use std::{collections::BTreeMap, sync::OnceLock};

use brk_computer::Computer;
use brk_error::{Error, Result};
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_structs::Height;
use nucleo_matcher::{
    Config, Matcher,
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
};
use quick_cache::sync::Cache;
use tabled::settings::Style;
use vecdb::{AnyCollectableVec, AnyStoredVec};

mod deser;
mod format;
mod ids;
mod index;
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

pub fn cached_errors() -> &'static Cache<String, String> {
    static CACHE: OnceLock<Cache<String, String>> = OnceLock::new();
    CACHE.get_or_init(|| Cache::new(1000))
}

#[allow(dead_code)]
pub struct Interface<'a> {
    vecs: Vecs<'a>,
    parser: &'a Parser,
    indexer: &'a Indexer,
    computer: &'a Computer,
}

impl<'a> Interface<'a> {
    pub fn build(parser: &Parser, indexer: &Indexer, computer: &Computer) -> Self {
        let parser = parser.static_clone();
        let indexer = indexer.static_clone();
        let computer = computer.static_clone();
        let vecs = Vecs::build(indexer, computer);

        Self {
            vecs,
            parser,
            indexer,
            computer,
        }
    }

    pub fn get_height(&self) -> Height {
        Height::from(self.indexer.vecs.height_to_blockhash.stamp())
    }

    pub fn search(&self, params: &Params) -> Result<Vec<(String, &&dyn AnyCollectableVec)>> {
        let ids = &params.ids;
        let index = params.index;

        let ids_to_vec = self
            .vecs
            .index_to_id_to_vec
            .get(&index)
            .ok_or(Error::String(format!(
                "Index \"{}\" isn't a valid index",
                index
            )))?;

        ids.iter()
            .map(|id| {
                let vec = ids_to_vec.get(id.as_str()).ok_or_else(|| {
                    let cached_errors = cached_errors();

                    if let Some(message) = cached_errors.get(id) {
                        return Error::String(message)
                    }

                    let mut message = format!(
                        "No vec named \"{}\" indexed by \"{}\" found.\n",
                        id,
                        index
                    );

                    let mut matcher = Matcher::new(Config::DEFAULT);

                    let matches = Pattern::new(
                        id.as_str(),
                        CaseMatching::Ignore,
                        Normalization::Smart,
                        AtomKind::Fuzzy,
                    )
                    .match_list(ids_to_vec.keys(), &mut matcher)
                    .into_iter()
                    .take(10)
                    .map(|(s, _)| s)
                    .collect::<Vec<_>>();

                    if !matches.is_empty() {
                        message +=
                            &format!("\nMaybe you meant one of the following: {matches:#?} ?\n");
                    }

                    if let Some(index_to_vec) = self.id_to_index_to_vec().get(id.as_str()) {
                        message += &format!("\nBut there is a vec named {id} which supports the following indexes: {:#?}\n", index_to_vec.keys());
                    }

                    cached_errors.insert(id.clone(), message.clone());

                    Error::String(message)
                });
                vec.map(|vec| (id.clone(), vec))
            })
            .collect::<Result<Vec<_>>>()
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
        self.format(self.search(&params)?, &params.rest)
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

    pub fn parser(&self) -> &Parser {
        self.parser
    }

    pub fn indexer(&self) -> &Indexer {
        self.indexer
    }

    pub fn computer(&self) -> &Computer {
        self.computer
    }
}
