use crate::{Etag, Output, OutputLegacy};

/// Metric output with metadata for caching.
#[derive(Debug)]
pub struct MetricOutput {
    pub output: Output,
    pub version: u64,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}

impl MetricOutput {
    pub fn etag(&self) -> Etag {
        Etag::from_metric(self.version, self.total, self.start, self.end)
    }
}

/// Deprecated: Legacy metric output with metadata for caching.
#[derive(Debug)]
pub struct MetricOutputLegacy {
    pub output: OutputLegacy,
    pub version: u64,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}

impl MetricOutputLegacy {
    pub fn etag(&self) -> Etag {
        Etag::from_metric(self.version, self.total, self.start, self.end)
    }
}
