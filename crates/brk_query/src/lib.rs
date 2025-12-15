#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

use std::{collections::BTreeMap, sync::Arc};

use brk_computer::Computer;
use brk_error::{Error, Result};
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_traversable::TreeNode;
use brk_types::{Format, Height, Index, IndexInfo, Limit, Metric, MetricCount};
use vecdb::{AnyExportableVec, AnyStoredVec};

// Infrastructure modules
#[cfg(feature = "tokio")]
mod r#async;
mod output;
mod vecs;

// Query impl blocks (extend Query with domain methods)
mod r#impl;

// Re-exports
#[cfg(feature = "tokio")]
pub use r#async::*;
pub use brk_types::{
    DataRange, DataRangeFormat, MetricSelection, MetricSelectionLegacy, PaginatedMetrics,
    Pagination, PaginationIndex,
};
pub use r#impl::BLOCK_TXS_PAGE_SIZE;
pub use output::{Output, Value};

use crate::vecs::{IndexToVec, MetricToVec};
use vecs::Vecs;

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

    /// Current indexed height
    pub fn height(&self) -> Height {
        Height::from(self.indexer().vecs.block.height_to_blockhash.stamp())
    }

    // === Metrics methods ===

    pub fn match_metric(&self, metric: &Metric, limit: Limit) -> Vec<&'static str> {
        self.vecs().matches(metric, limit)
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
        params: &DataRangeFormat,
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

    /// Search for vecs matching the given metrics and index
    pub fn search(&self, params: &MetricSelection) -> Vec<&'static dyn AnyExportableVec> {
        params
            .metrics
            .iter()
            .filter_map(|metric| self.vecs().get(metric, params.index))
            .collect()
    }

    /// Calculate total weight of the vecs for the given range
    pub fn weight(vecs: &[&dyn AnyExportableVec], from: Option<i64>, to: Option<i64>) -> usize {
        vecs.iter().map(|v| v.range_weight(from, to)).sum()
    }

    pub fn search_and_format(&self, params: MetricSelection) -> Result<Output> {
        let vecs = self.search(&params);

        if vecs.is_empty() {
            return Ok(Output::default(params.range.format()));
        }

        self.format(vecs.iter().collect(), &params.range)
    }

    /// Search and format with weight limit (for DDoS prevention)
    pub fn search_and_format_checked(
        &self,
        params: MetricSelection,
        max_weight: usize,
    ) -> Result<Output> {
        let vecs = self.search(&params);

        if vecs.is_empty() {
            return Ok(Output::default(params.range.format()));
        }

        let weight = Self::weight(&vecs, params.from(), params.to());
        if weight > max_weight {
            return Err(Error::String(format!(
                "Request too heavy: {weight} bytes exceeds limit of {max_weight} bytes"
            )));
        }

        self.format(vecs.iter().collect(), &params.range)
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

    pub fn get_metrics(&self, pagination: Pagination) -> PaginatedMetrics {
        self.vecs().metrics(pagination)
    }

    pub fn get_metrics_catalog(&self) -> &TreeNode {
        self.vecs().catalog()
    }

    pub fn get_index_to_vecids(&self, paginated_index: PaginationIndex) -> Option<&[&str]> {
        self.vecs().index_to_ids(paginated_index)
    }

    pub fn metric_to_indexes(&self, metric: Metric) -> Option<&Vec<Index>> {
        self.vecs().metric_to_indexes(metric)
    }

    // === Core accessors ===

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
