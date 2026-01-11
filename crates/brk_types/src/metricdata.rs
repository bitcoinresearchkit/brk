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
    /// Write metric data as JSON to buffer: `{"total":N,"start":N,"end":N,"data":[...]}`
    pub fn serialize(
        vec: &dyn AnySerializableVec,
        start: Option<usize>,
        end: Option<usize>,
        buf: &mut Vec<u8>,
    ) -> vecdb::Result<()> {
        let total = vec.len();
        let start_idx = start.unwrap_or(0);
        let end_idx = end.unwrap_or(total).min(total);

        write!(
            buf,
            r#"{{"total":{total},"start":{start_idx},"end":{end_idx},"data":"#
        )?;
        vec.write_json(start, end, buf)?;
        buf.push(b'}');
        Ok(())
    }
}
