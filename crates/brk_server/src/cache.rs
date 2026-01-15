use axum::http::HeaderMap;

use crate::{VERSION, extended::HeaderMapExtended};

/// Cache strategy for HTTP responses.
pub enum CacheStrategy {
    /// Data that changes with each new block (addresses, mining stats, txs, outspends)
    /// Etag = VERSION-{height}, Cache-Control: must-revalidate
    Height,

    /// Static/immutable data (blocks by hash, validate-address, metrics catalog)
    /// Etag = VERSION only, Cache-Control: must-revalidate
    Static,

    /// Volatile data (mempool) - no etag, just max-age
    /// Cache-Control: max-age={seconds}
    MaxAge(u64),
}

/// Resolved cache parameters
pub struct CacheParams {
    pub etag: Option<String>,
    pub cache_control: String,
}

impl CacheParams {
    /// Cache params using VERSION as etag
    pub fn version() -> Self {
        Self::resolve(&CacheStrategy::Static, || unreachable!())
    }

    pub fn etag_str(&self) -> &str {
        self.etag.as_deref().unwrap_or("")
    }

    pub fn matches_etag(&self, headers: &HeaderMap) -> bool {
        self.etag
            .as_ref()
            .is_some_and(|etag| headers.has_etag(etag))
    }

    pub fn resolve(strategy: &CacheStrategy, height: impl FnOnce() -> u32) -> Self {
        use CacheStrategy::*;
        match strategy {
            Height => Self {
                etag: Some(format!("{VERSION}-{}", height())),
                cache_control: "public, max-age=1, must-revalidate".into(),
            },
            Static => Self {
                etag: Some(VERSION.to_string()),
                cache_control: "public, max-age=1, must-revalidate".into(),
            },
            MaxAge(secs) => Self {
                etag: None,
                cache_control: format!("public, max-age={secs}"),
            },
        }
    }
}
