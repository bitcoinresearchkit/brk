use std::fmt;

/// HTTP ETag value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Etag(String);

impl Etag {
    /// Create from raw string
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Get inner string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume and return inner string
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Etag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Etag {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Etag {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Etag {
    /// Create ETag from metric data response info.
    ///
    /// Format varies based on whether the slice touches the end:
    /// - Slice ends before total: `{version:x}-{start}-{end}` (len irrelevant, data won't change if metric grows)
    /// - Slice reaches the end: `{version:x}-{start}-{total}` (len matters, new data would change results)
    ///
    /// `version` is the metric version for single queries, or the sum of versions for bulk queries.
    pub fn from_metric(version: u64, total: usize, start: usize, end: usize) -> Self {
        if end < total {
            // Fixed window not at the end - len doesn't matter
            Self(format!("{version:x}-{start}-{end}"))
        } else {
            // Fetching up to current end - len matters
            Self(format!("{version:x}-{start}-{total}"))
        }
    }
}
