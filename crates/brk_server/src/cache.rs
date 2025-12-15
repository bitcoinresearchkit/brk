use axum::http::HeaderMap;

use crate::{VERSION, extended::HeaderMapExtended};

/// Minimum confirmations before data is considered immutable
pub const MIN_CONFIRMATIONS: u32 = 6;

/// Cache strategy for HTTP responses.
///
/// # Future optimization: Immutable caching for blocks/txs
///
/// The `Immutable` variant supports caching deeply-confirmed blocks/txs forever
/// (1 year, `immutable` directive). To use it, you need the confirmation count:
///
/// ```ignore
/// // Example: cache block by hash as immutable if deeply confirmed
/// let confirmations = current_height - block_height + 1;
/// let prefix = *BlockHashPrefix::from(&hash);
/// state.cached_json(&headers, CacheStrategy::immutable(prefix, confirmations), |q| q.block(&hash)).await
/// ```
///
/// Currently all block/tx handlers use `Height` for simplicity since determining
/// confirmations requires knowing the block height upfront (an extra lookup).
/// This could be optimized by either:
/// 1. Including confirmation count in the response type
/// 2. Doing a lightweight height lookup before the main query
pub enum CacheStrategy {
    /// Immutable data (blocks by hash with 6+ confirmations)
    /// Etag = VERSION-{prefix:x}, Cache-Control: immutable, 1yr
    /// Falls back to Height if < 6 confirmations
    Immutable { prefix: u64, confirmations: u32 },

    /// Data that changes with each new block (addresses, block-by-height)
    /// Etag = VERSION-{height}, Cache-Control: must-revalidate
    Height,

    /// Data that changes with height + depends on parameter
    /// Etag = VERSION-{height}-{suffix}, Cache-Control: must-revalidate
    HeightWith(String),

    /// Static data (validate-address, metrics catalog)
    /// Etag = VERSION only, Cache-Control: 1hr
    Static,

    /// Volatile data (mempool) - no etag, just max-age
    /// Cache-Control: max-age={seconds}
    MaxAge(u64),
}

impl CacheStrategy {
    /// Create Immutable strategy - pass *prefix (deref BlockHashPrefix/TxidPrefix to u64)
    pub fn immutable(prefix: u64, confirmations: u32) -> Self {
        Self::Immutable {
            prefix,
            confirmations,
        }
    }

    /// Create HeightWith from any Display type
    pub fn height_with(suffix: impl std::fmt::Display) -> Self {
        Self::HeightWith(suffix.to_string())
    }
}

/// Resolved cache parameters
pub struct CacheParams {
    pub etag: Option<String>,
    pub cache_control: String,
}

impl CacheParams {
    pub fn etag_str(&self) -> &str {
        self.etag.as_deref().unwrap_or("")
    }

    pub fn matches_etag(&self, headers: &HeaderMap) -> bool {
        self.etag.as_ref().is_some_and(|etag| headers.has_etag(etag))
    }

    pub fn resolve(strategy: &CacheStrategy, height: impl FnOnce() -> u32) -> Self {
        use CacheStrategy::*;
        match strategy {
            Immutable {
                prefix,
                confirmations,
            } if *confirmations >= MIN_CONFIRMATIONS => Self {
                etag: Some(format!("{VERSION}-{prefix:x}")),
                cache_control: "public, max-age=31536000, immutable".into(),
            },
            Immutable { .. } | Height => Self {
                etag: Some(format!("{VERSION}-{}", height())),
                cache_control: "public, max-age=1, must-revalidate".into(),
            },
            HeightWith(suffix) => Self {
                etag: Some(format!("{VERSION}-{}-{suffix}", height())),
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
