use std::{collections::BTreeMap, sync::LazyLock};

use brk_error::{Error, Result};
use brk_traversable::TreeNode;
use brk_types::{
    BlockHashPrefix, Date, DetailedSeriesCount, Epoch, Format, Halving, Height, Index,
    IndexInfo, LegacyValue, Limit, Output, OutputLegacy, PaginatedSeries, Pagination,
    PaginationIndex, RangeIndex, RangeMap, SearchQuery, SeriesData, SeriesInfo, SeriesName,
    SeriesOutput, SeriesOutputLegacy, SeriesSelection, Timestamp, Version,
};
use parking_lot::RwLock;
use vecdb::{AnyExportableVec, ReadableVec};

use crate::{
    Query,
    vecs::{IndexToVec, SeriesToVec},
};

/// Monotonic block timestamps → height. Lazily extended as new blocks are indexed.
static HEIGHT_BY_MONOTONIC_TIMESTAMP: LazyLock<RwLock<RangeMap<Timestamp, Height>>> =
    LazyLock::new(|| RwLock::new(RangeMap::default()));

/// Estimated bytes per column header
const CSV_HEADER_BYTES_PER_COL: usize = 10;
/// Estimated bytes per cell value
const CSV_CELL_BYTES: usize = 15;

impl Query {
    pub fn search_series(&self, query: &SearchQuery) -> Vec<&'static str> {
        self.vecs().matches(&query.q, query.limit)
    }

    pub fn series_not_found_error(&self, series: &SeriesName) -> Error {
        // Check if series exists but with different indexes
        if let Some(indexes) = self.vecs().series_to_indexes(series.clone()) {
            let supported = indexes
                .iter()
                .map(|i| format!("/api/series/{series}/{}", i.name()))
                .collect::<Vec<_>>()
                .join(", ");
            return Error::SeriesUnsupportedIndex {
                series: series.to_string(),
                supported,
            };
        }

        // Series doesn't exist, suggest alternatives
        let matches = self
            .vecs()
            .matches(series, Limit::DEFAULT)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Error::SeriesNotFound(brk_error::SeriesNotFound::new(series.to_string(), matches))
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

        // Single-column fast path: stream directly, no Vec<T> materialization
        if num_cols == 1 {
            let col = columns[0];
            csv.push_str(col.name());
            csv.push('\n');
            col.write_csv_column(Some(start), Some(end), &mut csv)?;
            return Ok(csv);
        }

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

    /// Returns the latest value for a single series as a JSON value.
    pub fn latest(&self, series: &SeriesName, index: Index) -> Result<serde_json::Value> {
        let vec = self
            .vecs()
            .get(series, index)
            .ok_or_else(|| self.series_not_found_error(series))?;
        vec.last_json_value().ok_or(Error::NoData)
    }

    /// Returns the length (total data points) for a single series.
    pub fn len(&self, series: &SeriesName, index: Index) -> Result<usize> {
        let vec = self
            .vecs()
            .get(series, index)
            .ok_or_else(|| self.series_not_found_error(series))?;
        Ok(vec.len())
    }

    /// Returns the version for a single series.
    pub fn version(&self, series: &SeriesName, index: Index) -> Result<Version> {
        let vec = self
            .vecs()
            .get(series, index)
            .ok_or_else(|| self.series_not_found_error(series))?;
        Ok(vec.version())
    }

    /// Search for vecs matching the given series and index.
    /// Returns error if no series requested or any requested series is not found.
    pub fn search(&self, params: &SeriesSelection) -> Result<Vec<&'static dyn AnyExportableVec>> {
        if params.series.is_empty() {
            return Err(Error::NoSeries);
        }
        let mut vecs = Vec::with_capacity(params.series.len());
        for series in params.series.iter() {
            match self.vecs().get(series, params.index) {
                Some(vec) => vecs.push(vec),
                None => return Err(self.series_not_found_error(series)),
            }
        }
        Ok(vecs)
    }

    /// Calculate total weight of the vecs for the given range.
    pub fn weight(vecs: &[&dyn AnyExportableVec], from: Option<i64>, to: Option<i64>) -> usize {
        vecs.iter().map(|v| v.range_weight(from, to)).sum()
    }

    /// Resolve query metadata without formatting (cheap).
    /// Use with `format` for lazy formatting after ETag check.
    pub fn resolve(&self, params: SeriesSelection, max_weight: usize) -> Result<ResolvedQuery> {
        let vecs = self.search(&params)?;

        let total = vecs.iter().map(|v| v.len()).min().unwrap_or(0);
        let version: Version = vecs.iter().map(|v| v.version()).sum();
        let index = params.index;

        let start = match params.start() {
            Some(ri) => {
                let i = self.range_index_to_i64(ri, index)?;
                vecs.iter().map(|v| v.i64_to_usize(i)).min().unwrap_or(0)
            }
            None => 0,
        };

        let end = match params.end() {
            Some(ri) => {
                let i = self.range_index_to_i64(ri, index)?;
                vecs.iter()
                    .map(|v| v.i64_to_usize(i))
                    .min()
                    .unwrap_or(total)
            }
            None => params
                .limit()
                .map(|l| (start + *l).min(total))
                .unwrap_or(total),
        };

        let end = end.max(start);
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
            index: params.index,
            version,
            total,
            start,
            end,
            hash_prefix: self.tip_hash_prefix(),
        })
    }

    /// Format a resolved query (expensive).
    /// Call after ETag/cache checks to avoid unnecessary work.
    pub fn format(&self, resolved: ResolvedQuery) -> Result<SeriesOutput> {
        let ResolvedQuery {
            vecs,
            format,
            index,
            version,
            total,
            start,
            end,
            ..
        } = resolved;

        let output = match format {
            Format::CSV => Output::CSV(Self::columns_to_csv(&vecs, start, end)?),
            Format::JSON => {
                let count = end.saturating_sub(start);
                if vecs.len() == 1 {
                    let mut buf = Vec::with_capacity(count * 12 + 256);
                    SeriesData::serialize(vecs[0], index, start, end, &mut buf)?;
                    Output::Json(buf)
                } else {
                    let mut buf = Vec::with_capacity(count * 12 * vecs.len() + 256);
                    buf.push(b'[');
                    for (i, vec) in vecs.iter().enumerate() {
                        if i > 0 {
                            buf.push(b',');
                        }
                        SeriesData::serialize(*vec, index, start, end, &mut buf)?;
                    }
                    buf.push(b']');
                    Output::Json(buf)
                }
            }
        };

        Ok(SeriesOutput {
            output,
            version,
            total,
            start,
            end,
        })
    }

    /// Format a resolved query as raw data (just the JSON array, no SeriesData wrapper).
    /// CSV output is identical to `format` (no wrapper distinction for CSV).
    pub fn format_raw(&self, resolved: ResolvedQuery) -> Result<SeriesOutput> {
        if resolved.format() == Format::CSV {
            return self.format(resolved);
        }

        let ResolvedQuery {
            vecs,
            version,
            total,
            start,
            end,
            ..
        } = resolved;

        let count = end.saturating_sub(start);
        let mut buf = Vec::with_capacity(count * 12 + 2);
        vecs[0].write_json(Some(start), Some(end), &mut buf)?;

        Ok(SeriesOutput {
            output: Output::Json(buf),
            version,
            total,
            start,
            end,
        })
    }

    pub fn series_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        &self.vecs().series_to_index_to_vec
    }

    pub fn index_to_series_to_vec(&self) -> &BTreeMap<Index, SeriesToVec<'_>> {
        &self.vecs().index_to_series_to_vec
    }

    pub fn series_count(&self) -> DetailedSeriesCount {
        DetailedSeriesCount {
            total: self.vecs().counts.clone(),
            by_db: self.vecs().counts_by_db.clone(),
        }
    }

    pub fn indexes(&self) -> &[IndexInfo] {
        &self.vecs().indexes
    }

    pub fn series_list(&self, pagination: Pagination) -> PaginatedSeries {
        self.vecs().series(pagination)
    }

    pub fn series_catalog(&self) -> &TreeNode {
        self.vecs().catalog()
    }

    pub fn index_to_vecids(&self, paginated_index: PaginationIndex) -> Option<&[&str]> {
        self.vecs().index_to_ids(paginated_index)
    }

    pub fn series_info(&self, series: &SeriesName) -> Option<SeriesInfo> {
        let index_to_vec = self
            .vecs()
            .series_to_index_to_vec
            .get(series.replace("-", "_").as_str())?;
        let value_type = index_to_vec.values().next()?.value_type_to_string();
        let indexes = index_to_vec.keys().copied().collect();
        Some(SeriesInfo {
            indexes,
            value_type: value_type.into(),
        })
    }

    pub fn series_to_indexes(&self, series: SeriesName) -> Option<&Vec<Index>> {
        self.vecs().series_to_indexes(series)
    }

    /// Resolve a RangeIndex to an i64 offset for the given index type.
    fn range_index_to_i64(&self, ri: RangeIndex, index: Index) -> Result<i64> {
        match ri {
            RangeIndex::Int(i) => Ok(i),
            RangeIndex::Date(date) => self.date_to_i64(date, index),
            RangeIndex::Timestamp(ts) => self.timestamp_to_i64(ts, index),
        }
    }

    fn date_to_i64(&self, date: Date, index: Index) -> Result<i64> {
        // Direct date-based index conversion (day1, week1, month1, etc.)
        if let Some(idx) = index.date_to_index(date) {
            return Ok(idx as i64);
        }
        // Fall through to timestamp-based resolution (height, epoch, halving)
        self.timestamp_to_i64(Timestamp::from(date), index)
    }

    fn timestamp_to_i64(&self, ts: Timestamp, index: Index) -> Result<i64> {
        // Direct timestamp-based index conversion (minute10, hour1, etc.)
        if let Some(idx) = index.timestamp_to_index(ts) {
            return Ok(idx as i64);
        }
        // Height-based indexes: find block height, then convert
        let height = Height::from(self.height_for_timestamp(ts));
        match index {
            Index::Height => Ok(usize::from(height) as i64),
            Index::Epoch => Ok(usize::from(Epoch::from(height)) as i64),
            Index::Halving => Ok(usize::from(Halving::from(height)) as i64),
            _ => Err(Error::Parse(format!(
                "date/timestamp ranges not supported for index '{index}'"
            ))),
        }
    }

    /// Find the first block height at or after a given timestamp.
    /// O(log n) binary search. Lazily rebuilt as new blocks arrive.
    fn height_for_timestamp(&self, ts: Timestamp) -> usize {
        let current_height: usize = self.height().into();

        // Fast path: read lock, ceil is &self
        {
            let map = HEIGHT_BY_MONOTONIC_TIMESTAMP.read();
            if map.len() > current_height {
                return map.ceil(ts).map(usize::from).unwrap_or(current_height);
            }
        }

        // Slow path: rebuild from computer's precomputed monotonic timestamps
        let mut map = HEIGHT_BY_MONOTONIC_TIMESTAMP.write();
        if map.len() <= current_height {
            *map = RangeMap::from(self.computer().indexes.timestamp.monotonic.collect());
        }
        map.ceil(ts).map(usize::from).unwrap_or(current_height)
    }

    /// Deprecated - format a resolved query as legacy output (expensive).
    pub fn format_legacy(&self, resolved: ResolvedQuery) -> Result<SeriesOutputLegacy> {
        let ResolvedQuery {
            vecs,
            format,
            version,
            total,
            start,
            end,
            ..
        } = resolved;

        if vecs.is_empty() {
            return Ok(SeriesOutputLegacy {
                output: OutputLegacy::default(format),
                version: Version::ZERO,
                total: 0,
                start: 0,
                end: 0,
            });
        }

        let from = Some(start as i64);
        let to = Some(end as i64);

        let output = match format {
            Format::CSV => OutputLegacy::CSV(Self::columns_to_csv(&vecs, start, end)?),
            Format::JSON => {
                if vecs.len() == 1 {
                    let col = vecs[0];
                    let count = col.range_count(from, to);
                    let mut buf = Vec::new();
                    if count == 1 {
                        col.write_json_value(Some(start), &mut buf)?;
                        OutputLegacy::Json(LegacyValue::Value(buf))
                    } else {
                        col.write_json(Some(start), Some(end), &mut buf)?;
                        OutputLegacy::Json(LegacyValue::List(buf))
                    }
                } else {
                    let mut values = Vec::with_capacity(vecs.len());
                    for vec in &vecs {
                        let mut buf = Vec::new();
                        vec.write_json(Some(start), Some(end), &mut buf)?;
                        values.push(buf);
                    }
                    OutputLegacy::Json(LegacyValue::Matrix(values))
                }
            }
        };

        Ok(SeriesOutputLegacy {
            output,
            version,
            total,
            start,
            end,
        })
    }
}

/// A resolved series query ready for formatting.
/// Carries the vecs plus the metadata (version, total, end, hash_prefix) callers
/// need to derive an etag or cache policy.
pub struct ResolvedQuery {
    pub vecs: Vec<&'static dyn AnyExportableVec>,
    pub format: Format,
    pub index: Index,
    pub version: Version,
    pub total: usize,
    pub start: usize,
    pub end: usize,
    pub hash_prefix: BlockHashPrefix,
}

impl ResolvedQuery {
    pub fn format(&self) -> Format {
        self.format
    }

    pub fn csv_filename(&self) -> String {
        let names: Vec<_> = self.vecs.iter().map(|v| v.name()).collect();
        format!("{}-{}.csv", names.join("_"), self.index)
    }
}
