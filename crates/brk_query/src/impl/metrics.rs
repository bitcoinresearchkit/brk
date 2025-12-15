use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_traversable::TreeNode;
use brk_types::{
    Format, Index, IndexInfo, Limit, Metric, MetricCount, MetricData, PaginatedMetrics, Pagination,
    PaginationIndex,
};
use vecdb::AnyExportableVec;

use crate::vecs::{IndexToVec, MetricToVec};
use crate::{DataRangeFormat, MetricSelection, Output, Query};

/// Estimated bytes per column header
const CSV_HEADER_BYTES_PER_COL: usize = 10;
/// Estimated bytes per cell value
const CSV_CELL_BYTES: usize = 15;

impl Query {
    pub fn match_metric(&self, metric: &Metric, limit: Limit) -> Vec<&'static str> {
        self.vecs().matches(metric, limit)
    }

    pub fn metric_not_found_error(&self, metric: &Metric) -> Error {
        if let Some(first) = self.match_metric(metric, Limit::MIN).first() {
            Error::String(format!("Could not find '{metric}', did you mean '{first}'?"))
        } else {
            Error::String(format!("Could not find '{metric}'."))
        }
    }

    pub(crate) fn columns_to_csv(
        columns: &[&dyn AnyExportableVec],
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<String> {
        if columns.is_empty() {
            return Ok(String::new());
        }

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

    /// Format single metric - returns `MetricData`
    pub fn format(
        &self,
        metric: &dyn AnyExportableVec,
        params: &DataRangeFormat,
    ) -> Result<Output> {
        let from = params.from().map(|from| metric.i64_to_usize(from));
        let to = params.to().map(|to| metric.i64_to_usize(to));

        Ok(match params.format() {
            Format::CSV => Output::CSV(Self::columns_to_csv(
                &[metric],
                from.map(|v| v as i64),
                to.map(|v| v as i64),
            )?),
            Format::JSON => {
                let mut buf = Vec::new();
                MetricData::serialize(metric, from, to, &mut buf)?;
                Output::Json(buf)
            }
        })
    }

    /// Format multiple metrics - returns `Vec<MetricData>`
    pub fn format_bulk(
        &self,
        metrics: &[&dyn AnyExportableVec],
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
                metrics,
                from.map(|v| v as i64),
                to.map(|v| v as i64),
            )?),
            Format::JSON => {
                if metrics.is_empty() {
                    return Ok(Output::default(format));
                }

                let mut buf = Vec::new();
                buf.push(b'[');
                for (i, vec) in metrics.iter().enumerate() {
                    if i > 0 {
                        buf.push(b',');
                    }
                    MetricData::serialize(*vec, from, to, &mut buf)?;
                }
                buf.push(b']');
                Output::Json(buf)
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

    /// Search and format single metric
    pub fn search_and_format(&self, params: MetricSelection) -> Result<Output> {
        self.search_and_format_checked(params, usize::MAX)
    }

    /// Search and format single metric with weight limit
    pub fn search_and_format_checked(
        &self,
        params: MetricSelection,
        max_weight: usize,
    ) -> Result<Output> {
        let vecs = self.search(&params);

        let Some(metric) = vecs.first() else {
            let metric = params.metrics.first().cloned().unwrap_or_else(|| Metric::from(""));
            return Err(self.metric_not_found_error(&metric));
        };

        let weight = Self::weight(&vecs, params.from(), params.to());
        if weight > max_weight {
            return Err(Error::String(format!(
                "Request too heavy: {weight} bytes exceeds limit of {max_weight} bytes"
            )));
        }

        self.format(*metric, &params.range)
    }

    /// Search and format bulk metrics
    pub fn search_and_format_bulk(&self, params: MetricSelection) -> Result<Output> {
        self.search_and_format_bulk_checked(params, usize::MAX)
    }

    /// Search and format bulk metrics with weight limit (for DDoS prevention)
    pub fn search_and_format_bulk_checked(
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

        self.format_bulk(&vecs, &params.range)
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
