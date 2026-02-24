use std::{io::Write, ops::Deref};

use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use vecdb::AnySerializableVec;

use super::{Date, Index, Timestamp, Version};

/// Metric data with range information.
///
/// All metric data endpoints return this structure when format is JSON.
/// This type is not instantiated - use `MetricData::serialize()` to write JSON bytes directly.
#[derive(Debug, JsonSchema, Deserialize)]
pub struct MetricData<T = Value> {
    /// Version of the metric data
    pub version: Version,
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
        let version = u32::from(vec.version());
        let index_str = index.name();
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
    /// Returns `None` for non-date-based and sub-daily indexes (use `timestamps()` instead).
    pub fn dates(&self) -> Option<impl Iterator<Item = Date> + '_> {
        // Check first index to verify date conversion works (sub-daily returns None)
        self.index.index_to_date(self.start)?;
        let index = self.index;
        Some(self.indexes().map(move |i| {
            index.index_to_date(i).unwrap()
        }))
    }

    /// Returns an iterator over timestamps for the index range.
    /// Works for all date-based indexes including sub-daily.
    /// Returns `None` for non-date-based indexes.
    pub fn timestamps(&self) -> Option<impl Iterator<Item = Timestamp> + '_> {
        if !self.is_date_based() {
            return None;
        }
        let index = self.index;
        Some(self.indexes().map(move |i| {
            index.index_to_timestamp(i).unwrap()
        }))
    }

    /// Iterate over (index, &value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.indexes().zip(self.data.iter())
    }

    /// Iterate over (date, &value) pairs.
    /// Returns `None` for non-date-based and sub-daily indexes (use `iter_timestamps()` instead).
    pub fn iter_dates(&self) -> Option<impl Iterator<Item = (Date, &T)> + '_> {
        Some(self.dates()?.zip(self.data.iter()))
    }

    /// Iterate over (timestamp, &value) pairs.
    /// Works for all date-based indexes including sub-daily.
    /// Returns `None` for non-date-based indexes.
    pub fn iter_timestamps(&self) -> Option<impl Iterator<Item = (Timestamp, &T)> + '_> {
        Some(self.timestamps()?.zip(self.data.iter()))
    }
}

/// Metric data that is guaranteed to use a date-based index.
///
/// This is a newtype around `MetricData<T>` that guarantees `is_date_based()` is true,
/// making date methods infallible.
#[derive(Debug)]
pub struct DateMetricData<T>(MetricData<T>);

impl<T> DateMetricData<T> {
    /// Create a `DateMetricData` from a `MetricData`, returning `Err` if the index is not date-based.
    pub fn try_new(inner: MetricData<T>) -> Result<Self, MetricData<T>> {
        if inner.is_date_based() {
            Ok(Self(inner))
        } else {
            Err(inner)
        }
    }

    /// Consume and return the inner `MetricData`.
    pub fn into_inner(self) -> MetricData<T> {
        self.0
    }

    /// Returns an iterator over dates for the index range.
    /// Returns `None` for sub-daily indexes (use `timestamps()` instead).
    pub fn dates(&self) -> Option<impl Iterator<Item = Date> + '_> {
        self.0.dates()
    }

    /// Iterate over (date, &value) pairs.
    /// Returns `None` for sub-daily indexes (use `iter_timestamps()` instead).
    pub fn iter_dates(&self) -> Option<impl Iterator<Item = (Date, &T)> + '_> {
        self.0.iter_dates()
    }

    /// Returns an iterator over timestamps for the index range (infallible).
    /// Works for all date-based indexes including sub-daily.
    pub fn timestamps(&self) -> impl Iterator<Item = Timestamp> + '_ {
        self.0.timestamps().expect("DateMetricData is always date-based")
    }

    /// Iterate over (timestamp, &value) pairs (infallible).
    /// Works for all date-based indexes including sub-daily.
    pub fn iter_timestamps(&self) -> impl Iterator<Item = (Timestamp, &T)> + '_ {
        self.0.iter_timestamps().expect("DateMetricData is always date-based")
    }
}

impl<T> Deref for DateMetricData<T> {
    type Target = MetricData<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for DateMetricData<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = MetricData::<T>::deserialize(deserializer)?;
        Self::try_new(inner).map_err(|m| {
            serde::de::Error::custom(format!(
                "expected date-based index, got {:?}",
                m.index
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date_based_metric() -> MetricData<i32> {
        MetricData {
            version: Version::ONE,
            index: Index::Day1,
            total: 100,
            start: 0,
            end: 5,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![100, 200, 300, 400, 500],
        }
    }

    fn height_based_metric() -> MetricData<f64> {
        MetricData {
            version: Version::ONE,
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
    fn test_dates_for_day1() {
        let metric = date_based_metric();
        let dates: Vec<_> = metric.dates().unwrap().collect();
        assert_eq!(dates.len(), 5);
        // Day1 0 = Jan 3, 2009 (genesis)
        assert_eq!(dates[0].year(), 2009);
        assert_eq!(dates[0].month(), 1);
        assert_eq!(dates[0].day(), 3);
        // Day1 1 = Jan 9, 2009 (day one)
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
        let pairs: Vec<_> = metric.iter_dates().unwrap().collect();
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
    fn test_dates_returns_none_for_non_date_index() {
        let metric = height_based_metric();
        assert!(metric.dates().is_none());
    }

    #[test]
    fn test_iter_dates_returns_none_for_non_date_index() {
        let metric = height_based_metric();
        assert!(metric.iter_dates().is_none());
    }

    #[test]
    fn test_date_metric_data_try_new_ok() {
        let metric = date_based_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        assert_eq!(date_metric.data.len(), 5);
        let dates: Vec<_> = date_metric.dates().unwrap().collect();
        assert_eq!(dates.len(), 5);
        assert_eq!(dates[0].year(), 2009);
    }

    #[test]
    fn test_date_metric_data_try_new_err() {
        let metric = height_based_metric();
        assert!(DateMetricData::try_new(metric).is_err());
    }

    #[test]
    fn test_date_metric_data_iter_dates() {
        let metric = date_based_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        let pairs: Vec<_> = date_metric.iter_dates().unwrap().collect();
        assert_eq!(pairs.len(), 5);
        assert_eq!(pairs[0].0.day(), 3);
        assert_eq!(pairs[0].1, &100);
    }

    #[test]
    fn test_date_metric_data_deref() {
        let metric = date_based_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        // Access MetricData methods via Deref
        assert!(date_metric.is_date_based());
        assert_eq!(date_metric.indexes().count(), 5);
    }

    // Sub-daily tests

    fn sub_daily_metric() -> MetricData<f64> {
        MetricData {
            version: Version::ONE,
            index: Index::Hour1,
            total: 200000,
            start: 0,
            end: 3,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![10.0, 20.0, 30.0],
        }
    }

    #[test]
    fn test_sub_daily_is_date_based() {
        let metric = sub_daily_metric();
        assert!(metric.is_date_based());
    }

    #[test]
    fn test_sub_daily_dates_returns_none() {
        let metric = sub_daily_metric();
        assert!(metric.dates().is_none());
    }

    #[test]
    fn test_sub_daily_timestamps_returns_some() {
        let metric = sub_daily_metric();
        let ts: Vec<_> = metric.timestamps().unwrap().collect();
        assert_eq!(ts.len(), 3);
        // Hour1 index 0 = INDEX_EPOCH (2009-01-01 00:00:00 UTC)
        assert_eq!(*ts[0], 1230768000);
        // Hour1 index 1 = INDEX_EPOCH + 3600
        assert_eq!(*ts[1], 1230768000 + 3600);
    }

    #[test]
    fn test_sub_daily_iter_timestamps() {
        let metric = sub_daily_metric();
        let pairs: Vec<_> = metric.iter_timestamps().unwrap().collect();
        assert_eq!(pairs.len(), 3);
        assert_eq!(*pairs[0].0, 1230768000);
        assert_eq!(pairs[0].1, &10.0);
    }

    #[test]
    fn test_date_metric_data_sub_daily_timestamps() {
        let metric = sub_daily_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        // dates() returns None for sub-daily
        assert!(date_metric.dates().is_none());
        // timestamps() works for all date-based
        let ts: Vec<_> = date_metric.timestamps().collect();
        assert_eq!(ts.len(), 3);
    }

    #[test]
    fn test_date_metric_data_iter_timestamps() {
        let metric = sub_daily_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        let pairs: Vec<_> = date_metric.iter_timestamps().collect();
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[2].1, &30.0);
    }

    #[test]
    fn test_day1_timestamps_also_works() {
        // timestamps() works for daily indexes too
        let metric = date_based_metric();
        let ts: Vec<_> = metric.timestamps().unwrap().collect();
        assert_eq!(ts.len(), 5);
    }

    // Empty data

    fn empty_metric() -> MetricData<i32> {
        MetricData {
            version: Version::ONE,
            index: Index::Day1,
            total: 100,
            start: 5,
            end: 5,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![],
        }
    }

    #[test]
    fn test_empty_indexes() {
        let metric = empty_metric();
        assert_eq!(metric.indexes().count(), 0);
    }

    #[test]
    fn test_empty_iter() {
        let metric = empty_metric();
        assert_eq!(metric.iter().count(), 0);
    }

    #[test]
    fn test_empty_dates() {
        let metric = empty_metric();
        assert_eq!(metric.dates().unwrap().count(), 0);
    }

    #[test]
    fn test_empty_timestamps() {
        let metric = empty_metric();
        assert_eq!(metric.timestamps().unwrap().count(), 0);
    }

    // Non-date timestamps/iter_timestamps

    #[test]
    fn test_timestamps_returns_none_for_non_date() {
        let metric = height_based_metric();
        assert!(metric.timestamps().is_none());
    }

    #[test]
    fn test_iter_timestamps_returns_none_for_non_date() {
        let metric = height_based_metric();
        assert!(metric.iter_timestamps().is_none());
    }

    // DateMetricData sub-daily iter_dates returns None

    #[test]
    fn test_date_metric_data_sub_daily_iter_dates_returns_none() {
        let metric = sub_daily_metric();
        let date_metric = DateMetricData::try_new(metric).unwrap();
        assert!(date_metric.iter_dates().is_none());
    }

    // Month1 dates

    fn month1_metric() -> MetricData<i32> {
        MetricData {
            version: Version::ONE,
            index: Index::Month1,
            total: 200,
            start: 0,
            end: 3,
            stamp: "2024-01-01T00:00:00Z".to_string(),
            data: vec![1000, 2000, 3000],
        }
    }

    #[test]
    fn test_dates_for_month1() {
        let metric = month1_metric();
        let dates: Vec<_> = metric.dates().unwrap().collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0].year(), 2009);
        assert_eq!(dates[0].month(), 1);
        assert_eq!(dates[0].day(), 1);
        assert_eq!(dates[1].month(), 2);
        assert_eq!(dates[2].month(), 3);
    }

    #[test]
    fn test_timestamps_for_month1() {
        let metric = month1_metric();
        let ts: Vec<_> = metric.timestamps().unwrap().collect();
        assert_eq!(ts.len(), 3);
        // Each should be a valid timestamp
        assert!(*ts[0] > 0);
        assert!(*ts[1] > *ts[0]);
        assert!(*ts[2] > *ts[1]);
    }

    // Deserialize roundtrip

    #[test]
    fn test_date_metric_data_deserialize_valid() {
        let json = r#"{"version":1,"index":"day1","total":100,"start":0,"end":2,"stamp":"2024-01-01T00:00:00Z","data":[1,2]}"#;
        let result: Result<DateMetricData<i32>, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let dm = result.unwrap();
        assert_eq!(dm.data.len(), 2);
    }

    #[test]
    fn test_date_metric_data_deserialize_rejects_non_date() {
        let json = r#"{"version":1,"index":"height","total":100,"start":0,"end":2,"stamp":"2024-01-01T00:00:00Z","data":[1,2]}"#;
        let result: Result<DateMetricData<i32>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("date-based"), "error should mention date-based: {}", err);
    }

    // timestamp_to_index tests

    #[test]
    fn test_timestamp_to_index_hour1() {
        // INDEX_EPOCH + 2 hours
        let ts = Timestamp::new(1230768000 + 7200);
        assert_eq!(Index::Hour1.timestamp_to_index(ts), Some(2));
    }

    #[test]
    fn test_timestamp_to_index_minute5() {
        // INDEX_EPOCH + 15 minutes (= 3 * 5min intervals)
        let ts = Timestamp::new(1230768000 + 900);
        assert_eq!(Index::Minute5.timestamp_to_index(ts), Some(3));
    }

    #[test]
    fn test_timestamp_to_index_non_date_returns_none() {
        let ts = Timestamp::new(1230768000);
        assert!(Index::Height.timestamp_to_index(ts).is_none());
    }

    #[test]
    fn test_timestamp_to_index_day1_via_date_fallback() {
        // Day1 goes through date_to_index fallback
        // 2009-01-09 = Day1 index 1
        let ts = Timestamp::from(Date::new(2009, 1, 9));
        assert_eq!(Index::Day1.timestamp_to_index(ts), Some(1));
    }
}
