use crate::{Output, OutputLegacy, Version};

/// Metric output with metadata for caching.
#[derive(Debug)]
pub struct MetricOutput {
    pub output: Output,
    pub version: Version,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}

/// Deprecated: Legacy metric output with metadata for caching.
#[derive(Debug)]
pub struct MetricOutputLegacy {
    pub output: OutputLegacy,
    pub version: Version,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}
