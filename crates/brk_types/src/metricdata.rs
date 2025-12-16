use std::io::Write;

use schemars::JsonSchema;
use serde_json::Value;
use vecdb::AnySerializableVec;

/// Metric data with range information.
///
/// All metric data endpoints return this structure when format is JSON.
/// This type is not instantiated - use `MetricData::serialize()` to write JSON bytes directly.
#[derive(JsonSchema)]
pub struct MetricData {
    /// Total number of data points in the metric
    pub total: usize,
    /// Start index (inclusive) of the returned range
    pub from: usize,
    /// End index (exclusive) of the returned range
    pub to: usize,
    /// The metric data
    pub data: Vec<Value>,
}

impl MetricData {
    /// Write metric data as JSON to buffer: `{"total":N,"from":N,"to":N,"data":[...]}`
    pub fn serialize(
        vec: &dyn AnySerializableVec,
        from: Option<usize>,
        to: Option<usize>,
        buf: &mut Vec<u8>,
    ) -> vecdb::Result<()> {
        let total = vec.len();
        let from_idx = from.unwrap_or(0);
        let to_idx = to.unwrap_or(total).min(total);

        write!(
            buf,
            r#"{{"total":{total},"from":{from_idx},"to":{to_idx},"data":"#
        )?;
        vec.write_json(from, to, buf)?;
        buf.push(b'}');
        Ok(())
    }
}
