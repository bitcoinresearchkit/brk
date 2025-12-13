#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

use std::{collections::BTreeMap, sync::Arc};

use brk_computer::Computer;
use brk_error::{Error, Result};
use brk_indexer::Indexer;
use brk_monitor::Mempool;
use brk_reader::Reader;
use brk_traversable::TreeNode;
use brk_types::{
    Address, AddressStats, BlockInfo, BlockStatus, Format, Height, Index, IndexInfo, Limit,
    MempoolInfo, Metric, MetricCount, RecommendedFees, Transaction, TxStatus, Txid, TxidPath, Utxo,
};
use vecdb::{AnyExportableVec, AnyStoredVec};

#[cfg(feature = "tokio")]
mod r#async;
mod chain;
mod deser;
mod output;
mod pagination;
mod params;
mod vecs;

#[cfg(feature = "tokio")]
pub use r#async::*;
pub use output::{Output, Value};
pub use pagination::{PaginatedIndexParam, PaginatedMetrics, PaginationParam};
pub use params::{Params, ParamsDeprec, ParamsOpt};
use vecs::Vecs;

use crate::{
    chain::{
        get_address, get_address_txids, get_address_utxos, get_block_by_height,
        get_block_status_by_height, get_block_txids, get_blocks, get_height_by_hash,
        get_mempool_info, get_mempool_txids, get_recommended_fees, get_transaction,
        get_transaction_hex, get_transaction_status,
    },
    vecs::{IndexToVec, MetricToVec},
};

#[derive(Clone)]
pub struct Query(Arc<QueryInner<'static>>);
struct QueryInner<'a> {
    vecs: &'a Vecs<'a>,
    reader: Reader,
    indexer: &'a Indexer,
    computer: &'a Computer,
    mempool: Option<Mempool>,
}

impl Query {
    pub fn build(
        reader: &Reader,
        indexer: &Indexer,
        computer: &Computer,
        mempool: Option<Mempool>,
    ) -> Self {
        let reader = reader.clone();
        let indexer = Box::leak(Box::new(indexer.clone()));
        let computer = Box::leak(Box::new(computer.clone()));
        let vecs = Box::leak(Box::new(Vecs::build(indexer, computer)));

        Self(Arc::new(QueryInner {
            vecs,
            reader,
            indexer,
            computer,
            mempool,
        }))
    }

    pub fn get_height(&self) -> Height {
        Height::from(self.indexer().vecs.block.height_to_blockhash.stamp())
    }

    pub fn get_address(&self, address: Address) -> Result<AddressStats> {
        get_address(address, self)
    }

    pub fn get_address_txids(
        &self,
        address: Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        get_address_txids(address, after_txid, limit, self)
    }

    pub fn get_address_utxos(&self, address: Address) -> Result<Vec<Utxo>> {
        get_address_utxos(address, self)
    }

    pub fn get_transaction(&self, txid: TxidPath) -> Result<Transaction> {
        get_transaction(txid, self)
    }

    pub fn get_transaction_status(&self, txid: TxidPath) -> Result<TxStatus> {
        get_transaction_status(txid, self)
    }

    pub fn get_transaction_hex(&self, txid: TxidPath) -> Result<String> {
        get_transaction_hex(txid, self)
    }

    pub fn get_block(&self, hash: &str) -> Result<BlockInfo> {
        let height = get_height_by_hash(hash, self)?;
        get_block_by_height(height, self)
    }

    pub fn get_block_by_height(&self, height: Height) -> Result<BlockInfo> {
        get_block_by_height(height, self)
    }

    pub fn get_block_status(&self, hash: &str) -> Result<BlockStatus> {
        let height = get_height_by_hash(hash, self)?;
        get_block_status_by_height(height, self)
    }

    pub fn get_blocks(&self, start_height: Option<Height>) -> Result<Vec<BlockInfo>> {
        get_blocks(start_height, self)
    }

    pub fn get_block_txids(&self, hash: &str) -> Result<Vec<Txid>> {
        let height = get_height_by_hash(hash, self)?;
        get_block_txids(height, self)
    }

    pub fn get_mempool_info(&self) -> Result<MempoolInfo> {
        get_mempool_info(self)
    }

    pub fn get_mempool_txids(&self) -> Result<Vec<Txid>> {
        get_mempool_txids(self)
    }

    pub fn get_recommended_fees(&self) -> Result<RecommendedFees> {
        get_recommended_fees(self)
    }

    pub fn match_metric(&self, metric: &Metric, limit: Limit) -> Vec<&'static str> {
        self.vecs().matches(metric, limit)
    }

    pub fn search_metric_with_index(
        &self,
        metric: &str,
        index: Index,
        // params: &Params,
    ) -> Result<Vec<(String, &&dyn AnyExportableVec)>> {
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

    fn columns_to_csv(
        columns: &[&&dyn AnyExportableVec],
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<String> {
        if columns.is_empty() {
            return Ok(String::new());
        }

        let num_rows = columns[0].range_count(from, to);
        let num_cols = columns.len();

        let estimated_size = num_cols * 10 + num_rows * num_cols * 15;
        let mut csv = String::with_capacity(estimated_size);

        // Write headers from column names
        for (idx, col) in columns.iter().enumerate() {
            if idx > 0 {
                csv.push(',');
            }
            csv.push_str(col.name());
        }
        csv.push('\n');

        // Create one writer per column
        let mut writers: Vec<_> = columns
            .iter()
            .map(|col| col.create_writer(from, to))
            .collect();

        for _ in 0..num_rows {
            for (index, writer) in writers.iter_mut().enumerate() {
                if index > 0 {
                    csv.push(',');
                }
                writer.write_next(&mut csv)?;
            }
            csv.push('\n');
        }

        Ok(csv)
    }

    pub fn format(
        &self,
        metrics: Vec<&&dyn AnyExportableVec>,
        params: &ParamsOpt,
    ) -> Result<Output> {
        let from = params.from().map(|from| {
            metrics
                .iter()
                .map(|v| v.i64_to_usize(from))
                .min()
                .unwrap_or_default()
        });

        let to = params.to().map(|to| {
            metrics
                .iter()
                .map(|v| v.i64_to_usize(to))
                .min()
                .unwrap_or_default()
        });

        let format = params.format();

        Ok(match format {
            Format::CSV => Output::CSV(Self::columns_to_csv(
                &metrics,
                from.map(|v| v as i64),
                to.map(|v| v as i64),
            )?),
            Format::JSON => {
                let mut values = metrics
                    .iter()
                    .map(|vec| vec.collect_range_json_bytes(from, to).map_err(Error::from))
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

    pub fn metric_to_indexes(&self, metric: Metric) -> Option<&Vec<Index>> {
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
    pub fn mempool(&self) -> Option<&Mempool> {
        self.0.mempool.as_ref()
    }

    #[inline]
    pub fn vecs(&self) -> &'static Vecs<'static> {
        self.0.vecs
    }
}
