#![doc = include_str!("../README.md")]

use std::{collections::BTreeMap, sync::OnceLock};

use brk_computer::Computer;
use brk_error::{Error, Result};
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_structs::{Height, Index, IndexInfo};
use brk_traversable::TreeNode;
use nucleo_matcher::{
    Config, Matcher,
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
};
use quick_cache::sync::Cache;
use vecdb::{AnyCollectableVec, AnyStoredVec};

mod count;
mod deser;
mod format;
mod metrics;
mod output;
mod pagination;
mod params;
mod vecs;

pub use count::*;
pub use format::Format;
pub use output::{Output, Value};
pub use pagination::{PaginatedIndexParam, PaginatedMetrics, PaginationParam};
pub use params::{Params, ParamsDeprec, ParamsOpt};
use vecs::Vecs;

use crate::vecs::{IndexToVec, MetricToVec};

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
        let metrics = &params.metrics;
        let index = params.index;

        let ids_to_vec = self
            .vecs
            .index_to_metric_to_vec
            .get(&index)
            .ok_or(Error::String(format!(
                "Index \"{}\" isn't a valid index",
                index
            )))?;

        metrics.iter()
            .map(|metric| {
                let vec = ids_to_vec.get(metric.as_str()).ok_or_else(|| {
                    let cached_errors = cached_errors();

                    if let Some(message) = cached_errors.get(metric) {
                        return Error::String(message)
                    }

                    let mut message = format!(
                        "No vec named \"{}\" indexed by \"{}\" found.\n",
                        metric,
                        index
                    );

                    let mut matcher = Matcher::new(Config::DEFAULT);

                    let matches = Pattern::new(
                        metric.as_str(),
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

                    if let Some(index_to_vec) = self.metric_to_index_to_vec().get(metric.as_str()) {
                        message += &format!("\nBut there is a vec named {metric} which supports the following indexes: {:#?}\n", index_to_vec.keys());
                    }

                    cached_errors.insert(metric.clone(), message.clone());

                    Error::String(message)
                });
                vec.map(|vec| (metric.clone(), vec))
            })
            .collect::<Result<Vec<_>>>()
    }

    pub fn format(
        &self,
        metrics: Vec<(String, &&dyn AnyCollectableVec)>,
        params: &ParamsOpt,
    ) -> Result<Output> {
        let from = params.from().map(|from| {
            metrics
                .iter()
                .map(|(_, v)| v.i64_to_usize(from))
                .min()
                .unwrap_or_default()
        });

        let to = params.to().map(|to| {
            metrics
                .iter()
                .map(|(_, v)| v.i64_to_usize(to))
                .min()
                .unwrap_or_default()
        });

        let format = params.format();

        Ok(match format {
            Format::CSV => {
                let headers = metrics
                    .iter()
                    .map(|(id, _)| id.as_str())
                    .collect::<Vec<_>>();
                let mut values = metrics
                    .iter()
                    .map(|(_, vec)| Ok(vec.collect_range_string(from, to)?))
                    .collect::<Result<Vec<_>>>()?;

                if values.is_empty() {
                    return Ok(Output::CSV(headers.join(",")));
                }

                let first_len = values[0].len();
                let estimated_size = (headers.len() + values.len() * first_len) * 15;
                let mut csv = String::with_capacity(estimated_size);

                csv.push_str(&headers.join(","));
                csv.push('\n');

                for col_index in 0..first_len {
                    let mut first = true;
                    for vec in &mut values {
                        if col_index < vec.len() {
                            if !first {
                                csv.push(',');
                            }
                            first = false;

                            let field = std::mem::take(&mut vec[col_index]);

                            if field.contains(',') {
                                csv.push('"');
                                csv.push_str(&field);
                                csv.push('"');
                            } else {
                                csv.push_str(&field);
                            }
                        }
                    }
                    csv.push('\n');
                }

                Output::CSV(csv)
            }
            Format::JSON => {
                let mut values = metrics
                    .iter()
                    .map(|(_, vec)| -> Result<Vec<u8>> {
                        Ok(vec.collect_range_json_bytes(from, to)?)
                    })
                    .collect::<Result<Vec<_>>>()?;

                if values.is_empty() {
                    return Ok(Output::default(format));
                }

                if values.len() == 1 {
                    Output::Json(Value::List(values.pop().unwrap()))
                } else {
                    Output::Json(Value::Matrix(values))
                }
            }
        })
    }

    pub fn search_and_format(&self, params: Params) -> Result<Output> {
        self.format(self.search(&params)?, &params.rest)
    }

    pub fn metric_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        &self.vecs.metric_to_index_to_vec
    }

    pub fn index_to_metric_to_vec(&self) -> &BTreeMap<Index, MetricToVec<'_>> {
        &self.vecs.index_to_metric_to_vec
    }

    pub fn metric_count(&self) -> MetricCount {
        MetricCount {
            distinct_metrics: self.distinct_metric_count(),
            total_endpoints: self.total_metric_count(),
        }
    }

    pub fn distinct_metric_count(&self) -> usize {
        self.vecs.distinct_metric_count
    }

    pub fn total_metric_count(&self) -> usize {
        self.vecs.total_metric_count
    }

    pub fn get_indexes(&self) -> &[IndexInfo] {
        &self.vecs.indexes
    }

    pub fn get_metrics(&'static self, pagination: PaginationParam) -> PaginatedMetrics {
        self.vecs.metrics(pagination)
    }

    pub fn get_metrics_catalog(&self) -> &TreeNode {
        self.vecs.catalog.as_ref().unwrap()
    }

    pub fn get_index_to_vecids(&self, paginated_index: PaginatedIndexParam) -> Vec<&str> {
        self.vecs.index_to_ids(paginated_index)
    }

    pub fn metric_to_indexes(&self, metric: String) -> Option<&Vec<Index>> {
        self.vecs.metric_to_indexes(metric)
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
