#![doc = include_str!("../README.md")]

use std::{collections::BTreeMap, sync::Arc};

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::TreeNode;
use brk_types::{
    Address, AddressStats, Format, Height, Index, IndexInfo, Limit, Metric, MetricCount,
    Transaction, TxidPath,
};
use vecdb::{AnyCollectableVec, AnyStoredVec};

mod chain;
mod deser;
mod output;
mod pagination;
mod params;
mod vecs;

pub use output::{Output, Value};
pub use pagination::{PaginatedIndexParam, PaginatedMetrics, PaginationParam};
pub use params::{Params, ParamsDeprec, ParamsOpt};
use vecs::Vecs;

use crate::{
    chain::{get_address, get_transaction_info},
    vecs::{IndexToVec, MetricToVec},
};

#[derive(Clone)]
pub struct Query(Arc<QueryInner<'static>>);
struct QueryInner<'a> {
    vecs: &'a Vecs<'a>,
    reader: Reader,
    indexer: &'a Indexer,
    computer: &'a Computer,
}

impl Query {
    pub fn build(reader: &Reader, indexer: &Indexer, computer: &Computer) -> Self {
        let reader = reader.clone();
        let indexer = Box::leak(Box::new(indexer.clone()));
        let computer = Box::leak(Box::new(computer.clone()));
        let vecs = Box::leak(Box::new(Vecs::build(indexer, computer)));

        Self(Arc::new(QueryInner {
            vecs,
            reader,
            indexer,
            computer,
        }))
    }

    pub fn get_height(&self) -> Height {
        Height::from(self.indexer().vecs.height_to_blockhash.stamp())
    }

    pub fn get_address(&self, address: Address) -> Result<AddressStats> {
        get_address(address, self)
    }

    pub fn get_transaction_info(&self, txid: TxidPath) -> Result<Transaction> {
        get_transaction_info(txid, self)
    }

    pub fn match_metric(&self, metric: &Metric, limit: Limit) -> Vec<&str> {
        self.vecs().matches(metric, limit)
    }

    pub fn search_metric_with_index(
        &self,
        metric: &str,
        index: Index,
        // params: &Params,
    ) -> Result<Vec<(String, &&dyn AnyCollectableVec)>> {
        todo!();

        // let all_metrics = &self.vecs.metrics;
        // let metrics = &params.metrics;
        // let index = params.index;

        // let ids_to_vec = self
        //     .vecs
        //     .index_to_metric_to_vec
        //     .get(&index)
        //     .ok_or(Error::String(format!(
        //         "Index \"{}\" isn't a valid index",
        //         index
        //     )))?;

        // metrics
        //     .iter()
        //     .map(|metric| {
        //         let vec = ids_to_vec.get(metric.as_str()).ok_or_else(|| {
        //             let matches: Vec<&str> = MATCHER.with(|matcher| {
        //                 let matcher = matcher.borrow();
        //                 let mut scored: Vec<(&str, i64)> = all_metrics
        //                     .iter()
        //                     .filter_map(|m| matcher.fuzzy_match(m, metric).map(|s| (*m, s)))
        //                     .collect();

        //                 scored.sort_unstable_by_key(|&(_, s)| std::cmp::Reverse(s));
        //                 scored.into_iter().take(5).map(|(m, _)| m).collect()
        //             });

        //             let mut message = format!("No vec \"{metric}\" for index \"{index}\".\n");
        //             if !matches.is_empty() {
        //                 message += &format!("\nDid you mean: {matches:?}\n");
        //             }

        //             Error::String(message)
        //         });
        //         vec.map(|vec| (metric.clone(), vec))
        //     })
        //     .collect::<Result<Vec<_>>>()
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
                    .map(|(_, vec)| vec.collect_range_string(from, to))
                    .collect::<Vec<_>>();

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
                    .map(|(_, vec)| vec.collect_range_json_bytes(from, to))
                    .collect::<Vec<_>>();

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
        todo!()
        // self.format(self.search(&params)?, &params.rest)
    }

    pub fn metric_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        &self.vecs().metric_to_index_to_vec
    }

    pub fn index_to_metric_to_vec(&self) -> &BTreeMap<Index, MetricToVec<'_>> {
        &self.vecs().index_to_metric_to_vec
    }

    pub fn metric_count(&self) -> MetricCount {
        MetricCount {
            distinct_metrics: self.distinct_metric_count(),
            total_endpoints: self.total_metric_count(),
        }
    }

    pub fn distinct_metric_count(&self) -> usize {
        self.vecs().distinct_metric_count
    }

    pub fn total_metric_count(&self) -> usize {
        self.vecs().total_metric_count
    }

    pub fn get_indexes(&self) -> &[IndexInfo] {
        &self.vecs().indexes
    }

    pub fn get_metrics(&self, pagination: PaginationParam) -> PaginatedMetrics {
        self.vecs().metrics(pagination)
    }

    pub fn get_metrics_catalog(&self) -> &TreeNode {
        self.vecs().catalog()
    }

    pub fn get_index_to_vecids(&self, paginated_index: PaginatedIndexParam) -> Vec<&str> {
        self.vecs().index_to_ids(paginated_index)
    }

    pub fn metric_to_indexes(&self, metric: String) -> Option<&Vec<Index>> {
        self.vecs().metric_to_indexes(metric)
    }

    #[inline]
    pub fn reader(&self) -> &Reader {
        &self.0.reader
    }

    #[inline]
    pub fn indexer(&self) -> &Indexer {
        self.0.indexer
    }

    #[inline]
    pub fn computer(&self) -> &Computer {
        self.0.computer
    }

    #[inline]
    pub fn vecs(&self) -> &'static Vecs<'static> {
        self.0.vecs
    }
}
