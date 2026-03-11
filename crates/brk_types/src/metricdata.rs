use std::ops::Deref;

use schemars::JsonSchema;
use serde::{Deserialize, de::DeserializeOwned};
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
    /// Value type (e.g. "f32", "u64", "Sats")
    #[serde(rename = "type", default)]
    pub value_type: String,
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
        let total = vec.len();
        let end = end.min(total);
        let start = start.min(end);

        let mut itoa_buf = itoa::Buffer::new();

        buf.extend_from_slice(b"{\"version\":");
        buf.extend_from_slice(itoa_buf.format(u32::from(vec.version())).as_bytes());
        buf.extend_from_slice(b",\"index\":\"");
        buf.extend_from_slice(index.name().as_bytes());
        buf.extend_from_slice(b"\",\"type\":\"");
        buf.extend_from_slice(vec.value_type_to_string().as_bytes());
        buf.extend_from_slice(b"\",\"total\":");
        buf.extend_from_slice(itoa_buf.format(total).as_bytes());
        buf.extend_from_slice(b",\"start\":");
        buf.extend_from_slice(itoa_buf.format(start).as_bytes());
        buf.extend_from_slice(b",\"end\":");
        buf.extend_from_slice(itoa_buf.format(end).as_bytes());
        buf.extend_from_slice(b",\"stamp\":\"");
        buf.extend_from_slice(Timestamp::now().to_iso8601().as_bytes());
        buf.extend_from_slice(b"\",\"data\":");

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
        Some(self.indexes().map(move |i| index.index_to_date(i).unwrap()))
    }

    /// Returns an iterator over timestamps for the index range.
    /// Works for all date-based indexes including sub-daily.
    /// Returns `None` for non-date-based indexes.
    pub fn timestamps(&self) -> Option<impl Iterator<Item = Timestamp> + '_> {
        if !self.is_date_based() {
            return None;
        }
        let index = self.index;
        Some(
            self.indexes()
                .map(move |i| index.index_to_timestamp(i).unwrap()),
        )
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
        self.0
            .timestamps()
            .expect("DateMetricData is always date-based")
    }

    /// Iterate over (timestamp, &value) pairs (infallible).
    /// Works for all date-based indexes including sub-daily.
    pub fn iter_timestamps(&self) -> impl Iterator<Item = (Timestamp, &T)> + '_ {
        self.0
            .iter_timestamps()
            .expect("DateMetricData is always date-based")
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
            serde::de::Error::custom(format!("expected date-based index, got {:?}", m.index))
        })
    }
}
