use std::io::Write;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use vecdb::AnySerializableVec;

use super::{Index, Timestamp};

/// Metric data with range information.
///
/// All metric data endpoints return this structure when format is JSON.
/// This type is not instantiated - use `MetricData::serialize()` to write JSON bytes directly.
#[derive(Debug, JsonSchema, Deserialize)]
pub struct MetricData<T = Value> {
    /// Version of the metric data
    pub version: u64,
    /// The index type used for this query
    pub index: Index,
    /// Total number of data points in the metric
    pub total: usize,
    /// Start index (inclusive) of the returned range
    pub start: usize,
    /// End index (exclusive) of the returned range
    pub end: usize,
    /// ISO 8601 timestamp of when the response was generated
    pub stamp: String,
    /// The metric data
    pub data: Vec<T>,
}

impl MetricData {
    /// Write metric data as JSON to buffer: `{"version":N,"index":"...","total":N,"start":N,"end":N,"stamp":"...","data":[...]}`
    pub fn serialize(
        vec: &dyn AnySerializableVec,
        index: Index,
        start: usize,
        end: usize,
        buf: &mut Vec<u8>,
    ) -> vecdb::Result<()> {
        let version = u64::from(vec.version());
        let index_str = index.serialize_long();
        let total = vec.len();
        let end = end.min(total);
        let start = start.min(end);
        let stamp = Timestamp::now().to_iso8601();

        write!(
            buf,
            r#"{{"version":{version},"index":"{index_str}","total":{total},"start":{start},"end":{end},"stamp":"{stamp}","data":"#,
        )?;
        vec.write_json(Some(start), Some(end), buf)?;
        buf.push(b'}');
        Ok(())
    }
}

impl<T> MetricData<T> {
    /// Returns an iterator over the index range.
    pub fn indexes(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    /// Returns true if this metric uses a date-based index.
    pub fn is_date_based(&self) -> bool {
        self.index.is_date_based()
    }

    /// Returns an iterator over dates for the index range.
    /// Panics if the index is not date-based.
    pub fn dates(&self) -> impl Iterator<Item = super::Date> + '_ {
        let index = self.index;
        self.indexes().map(move |i| {
            index
                .index_to_date(i)
                .expect("dates() called on non-date-based index")
        })
    }

    /// Iterate over (index, &value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.indexes().zip(self.data.iter())
    }

    /// Iterate over (date, &value) pairs.
    /// Panics if the index is not date-based.
    pub fn iter_dates(&self) -> impl Iterator<Item = (super::Date, &T)> + '_ {
        self.dates().zip(self.data.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date_based_metric() -> MetricData<i32> {
        MetricData {
            version: 1,
            index: Index::DateIndex,
            total: 100,
            start: 0,
            end: 5,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![100, 200, 300, 400, 500],
        }
    }

    fn height_based_metric() -> MetricData<f64> {
        MetricData {
            version: 1,
            index: Index::Height,
            total: 1000,
            start: 800000,
            end: 800005,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![1.5, 2.5, 3.5, 4.5, 5.5],
        }
    }

    #[test]
    fn test_indexes_returns_range() {
        let metric = date_based_metric();
        let indexes: Vec<_> = metric.indexes().collect();
        assert_eq!(indexes, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_indexes_with_offset() {
        let metric = height_based_metric();
        let indexes: Vec<_> = metric.indexes().collect();
        assert_eq!(indexes, vec![800000, 800001, 800002, 800003, 800004]);
    }

    #[test]
    fn test_is_date_based_true() {
        let metric = date_based_metric();
        assert!(metric.is_date_based());
    }

    #[test]
    fn test_is_date_based_false() {
        let metric = height_based_metric();
        assert!(!metric.is_date_based());
    }

    #[test]
    fn test_dates_for_dateindex() {
        let metric = date_based_metric();
        let dates: Vec<_> = metric.dates().collect();
        assert_eq!(dates.len(), 5);
        // DateIndex 0 = Jan 3, 2009 (genesis)
        assert_eq!(dates[0].year(), 2009);
        assert_eq!(dates[0].month(), 1);
        assert_eq!(dates[0].day(), 3);
        // DateIndex 1 = Jan 9, 2009 (day one)
        assert_eq!(dates[1].year(), 2009);
        assert_eq!(dates[1].month(), 1);
        assert_eq!(dates[1].day(), 9);
    }

    #[test]
    fn test_iter() {
        let metric = date_based_metric();
        let pairs: Vec<_> = metric.iter().collect();
        assert_eq!(pairs.len(), 5);
        assert_eq!(pairs[0], (0, &100));
        assert_eq!(pairs[1], (1, &200));
        assert_eq!(pairs[4], (4, &500));
    }

    #[test]
    fn test_iter_with_offset() {
        let metric = height_based_metric();
        let pairs: Vec<_> = metric.iter().collect();
        assert_eq!(pairs.len(), 5);
        assert_eq!(pairs[0], (800000, &1.5));
        assert_eq!(pairs[4], (800004, &5.5));
    }

    #[test]
    fn test_iter_dates() {
        let metric = date_based_metric();
        let pairs: Vec<_> = metric.iter_dates().collect();
        assert_eq!(pairs.len(), 5);
        // First pair: (Jan 3 2009, 100)
        assert_eq!(pairs[0].0.year(), 2009);
        assert_eq!(pairs[0].0.month(), 1);
        assert_eq!(pairs[0].0.day(), 3);
        assert_eq!(pairs[0].1, &100);
        // Second pair: (Jan 9 2009, 200)
        assert_eq!(pairs[1].0.day(), 9);
        assert_eq!(pairs[1].1, &200);
    }

    #[test]
    #[should_panic(expected = "dates() called on non-date-based index")]
    fn test_dates_panics_for_non_date_index() {
        let metric = height_based_metric();
        let _: Vec<_> = metric.dates().collect();
    }

    #[test]
    #[should_panic(expected = "dates() called on non-date-based index")]
    fn test_iter_dates_panics_for_non_date_index() {
        let metric = height_based_metric();
        let _: Vec<_> = metric.iter_dates().collect();
    }
}
