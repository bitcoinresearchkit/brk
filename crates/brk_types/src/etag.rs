use std::fmt;

use super::{BlockHashPrefix, Version};

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
    /// Tail uses hash prefix (changes per-block and on reorgs),
    /// non-tail uses total (changes per-block).
    pub fn from_series(
        version: Version,
        total: usize,
        end: usize,
        hash_prefix: BlockHashPrefix,
    ) -> Self {
        let v = u32::from(version);
        if end >= total {
            let h = *hash_prefix;
            Self(format!("v{v}-{h:x}"))
        } else {
            Self(format!("v{v}-{total}"))
        }
    }
}
