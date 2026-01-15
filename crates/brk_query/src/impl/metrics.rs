use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_traversable::TreeNode;
use brk_types::{
    DetailedMetricCount, Format, Index, IndexInfo, Limit, Metric, MetricData, MetricOutput,
    MetricSelection, Output, PaginatedMetrics, Pagination, PaginationIndex,
};
use vecdb::AnyExportableVec;

use crate::{
    Query, ResolvedQuery,
    vecs::{IndexToVec, MetricToVec},
};

/// Estimated bytes per column header
const CSV_HEADER_BYTES_PER_COL: usize = 10;
/// Estimated bytes per cell value
const CSV_CELL_BYTES: usize = 15;

impl Query {
    pub fn match_metric(&self, metric: &Metric, limit: Limit) -> Vec<&'static str> {
        self.vecs().matches(metric, limit)
    }

    pub fn metric_not_found_error(&self, metric: &Metric) -> Error {
        // Check if metric exists but with different indexes
        if let Some(indexes) = self.vecs().metric_to_indexes(metric.clone()) {
            let index_list: Vec<_> = indexes.iter().map(|i| i.to_string()).collect();
            return Error::MetricUnsupportedIndex {
                metric: metric.to_string(),
                supported: index_list.join(", "),
            };
        }

        // Metric doesn't exist, suggest alternatives
        Error::MetricNotFound {
            metric: metric.to_string(),
            suggestion: self
                .match_metric(metric, Limit::MIN)
                .first()
                .map(|s| s.to_string()),
        }
    }

    pub(crate) fn columns_to_csv(
        columns: &[&dyn AnyExportableVec],
        start: usize,
        end: usize,
    ) -> Result<String> {
        if columns.is_empty() {
            return Ok(String::new());
        }

        let from = Some(start as i64);
        let to = Some(end as i64);

        let num_rows = columns[0].range_count(from, to);
        let num_cols = columns.len();

        let estimated_size =
            num_cols * CSV_HEADER_BYTES_PER_COL + num_rows * num_cols * CSV_CELL_BYTES;
        let mut csv = String::with_capacity(estimated_size);

        for (i, col) in columns.iter().enumerate() {
            if i > 0 {
                csv.push(',');
            }
            csv.push_str(col.name());
        }
        csv.push('\n');

        let mut writers: Vec<_> = columns
            .iter()
            .map(|col| col.create_writer(from, to))
            .collect();

        for _ in 0..num_rows {
            for (i, writer) in writers.iter_mut().enumerate() {
                if i > 0 {
                    csv.push(',');
                }
                writer.write_next(&mut csv)?;
            }
            csv.push('\n');
        }

        Ok(csv)
    }

    /// Search for vecs matching the given metrics and index.
    /// Returns error if no metrics requested or any requested metric is not found.
    pub fn search(&self, params: &MetricSelection) -> Result<Vec<&'static dyn AnyExportableVec>> {
        if params.metrics.is_empty() {
            return Err(Error::NoMetrics);
        }
        let mut vecs = Vec::with_capacity(params.metrics.len());
        for metric in params.metrics.iter() {
            match self.vecs().get(metric, params.index) {
                Some(vec) => vecs.push(vec),
                None => return Err(self.metric_not_found_error(metric)),
            }
        }
        Ok(vecs)
    }

    /// Calculate total weight of the vecs for the given range.
    /// Applies index-specific cost multipliers for rate limiting.
    pub fn weight(vecs: &[&dyn AnyExportableVec], from: Option<i64>, to: Option<i64>) -> usize {
        vecs.iter()
            .map(|v| {
                let base = v.range_weight(from, to);
                let multiplier = Index::try_from(v.index_type_to_string())
                    .map(|i| i.cost_multiplier())
                    .unwrap_or(1);
                base * multiplier
            })
            .sum()
    }

    /// Resolve query metadata without formatting (cheap).
    /// Use with `format` for lazy formatting after ETag check.
    pub fn resolve(
        &self,
        params: MetricSelection,
        max_weight: usize,
    ) -> Result<ResolvedQuery> {
        let vecs = self.search(&params)?;

        let total = vecs.iter().map(|v| v.len()).min().unwrap_or(0);
        let version: u64 = vecs.iter().map(|v| u64::from(v.version())).sum();

        let start = params
            .start()
            .map(|s| vecs.iter().map(|v| v.i64_to_usize(s)).min().unwrap_or(0))
            .unwrap_or(0);

        let end = params
            .end_for_len(total)
            .map(|e| {
                vecs.iter()
                    .map(|v| v.i64_to_usize(e))
                    .min()
                    .unwrap_or(total)
            })
            .unwrap_or(total);

        let weight = Self::weight(&vecs, Some(start as i64), Some(end as i64));
        if weight > max_weight {
            return Err(Error::WeightExceeded {
                requested: weight,
                max: max_weight,
            });
        }

        Ok(ResolvedQuery {
            vecs,
            format: params.format(),
            version,
            total,
            start,
            end,
        })
    }

    /// Format a resolved query (expensive).
    /// Call after ETag/cache checks to avoid unnecessary work.
    pub fn format(&self, resolved: ResolvedQuery) -> Result<MetricOutput> {
        let ResolvedQuery {
            vecs,
            format,
            version,
            total,
            start,
            end,
        } = resolved;

        let output = match format {
            Format::CSV => Output::CSV(Self::columns_to_csv(&vecs, start, end)?),
            Format::JSON => {
                if vecs.len() == 1 {
                    let mut buf = Vec::new();
                    MetricData::serialize(vecs[0], start, end, &mut buf)?;
                    Output::Json(buf)
                } else {
                    let mut buf = Vec::new();
                    buf.push(b'[');
                    for (i, vec) in vecs.iter().enumerate() {
                        if i > 0 {
                            buf.push(b',');
                        }
                        MetricData::serialize(*vec, start, end, &mut buf)?;
                    }
                    buf.push(b']');
                    Output::Json(buf)
                }
            }
        };

        Ok(MetricOutput {
            output,
            version,
            total,
            start,
            end,
        })
    }

    pub fn metric_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        &self.vecs().metric_to_index_to_vec
    }

    pub fn index_to_metric_to_vec(&self) -> &BTreeMap<Index, MetricToVec<'_>> {
        &self.vecs().index_to_metric_to_vec
    }

    pub fn metric_count(&self) -> DetailedMetricCount {
        DetailedMetricCount {
            total: self.vecs().counts.clone(),
            by_db: self.vecs().counts_by_db.clone(),
        }
    }

    pub fn indexes(&self) -> &[IndexInfo] {
        &self.vecs().indexes
    }

    pub fn metrics(&self, pagination: Pagination) -> PaginatedMetrics {
        self.vecs().metrics(pagination)
    }

    pub fn metrics_catalog(&self) -> &TreeNode {
        self.vecs().catalog()
    }

    pub fn index_to_vecids(&self, paginated_index: PaginationIndex) -> Option<&[&str]> {
        self.vecs().index_to_ids(paginated_index)
    }

    pub fn metric_to_indexes(&self, metric: Metric) -> Option<&Vec<Index>> {
        self.vecs().metric_to_indexes(metric)
    }
}
