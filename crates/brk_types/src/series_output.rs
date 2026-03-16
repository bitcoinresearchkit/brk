use crate::{Output, OutputLegacy, Version};

/// Series output with metadata for caching.
#[derive(Debug)]
pub struct SeriesOutput {
    pub output: Output,
    pub version: Version,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}

/// Deprecated: Legacy series output with metadata for caching.
#[derive(Debug)]
pub struct SeriesOutputLegacy {
    pub output: OutputLegacy,
    pub version: Version,
    pub total: usize,
    pub start: usize,
    pub end: usize,
}
