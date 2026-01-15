use std::io::Write;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use vecdb::AnySerializableVec;

/// Metric data with range information.
///
/// All metric data endpoints return this structure when format is JSON.
/// This type is not instantiated - use `MetricData::serialize()` to write JSON bytes directly.
#[derive(Debug, JsonSchema, Deserialize)]
pub struct MetricData<T = Value> {
    /// Version of the metric data
    pub version: u64,
    /// Total number of data points in the metric
    pub total: usize,
    /// Start index (inclusive) of the returned range
    pub start: usize,
    /// End index (exclusive) of the returned range
    pub end: usize,
    /// The metric data
    pub data: Vec<T>,
}

impl MetricData {
    /// Write metric data as JSON to buffer: `{"version":N,"total":N,"start":N,"end":N,"data":[...]}`
    pub fn serialize(
        vec: &dyn AnySerializableVec,
        start: usize,
        end: usize,
        buf: &mut Vec<u8>,
    ) -> vecdb::Result<()> {
        let version = u64::from(vec.version());
        let total = vec.len();
        let end = end.min(total);
        let start = start.min(end);

        write!(
            buf,
            r#"{{"version":{version},"total":{total},"start":{start},"end":{end},"data":"#,
        )?;
        vec.write_json(Some(start), Some(end), buf)?;
        buf.push(b'}');
        Ok(())
    }
}
