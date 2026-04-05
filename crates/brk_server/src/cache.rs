use axum::http::HeaderMap;
use brk_types::{BlockHashPrefix, Version};

use crate::{VERSION, extended::HeaderMapExtended};

/// Cache strategy for HTTP responses.
pub enum CacheStrategy {
    /// Chain-dependent data (addresses, mining stats, txs, outspends).
    /// Etag = {tip_hash_prefix:x}. Invalidates on any tip change including reorgs.
    Tip,

    /// Immutable data identified by hash in the URL (blocks by hash, confirmed tx data).
    /// Etag = {version}. Permanent; only bumped when response format changes.
    Immutable(Version),

    /// Static non-chain data (validate-address, series catalog, pool list).
    /// Etag = CARGO_PKG_VERSION. Invalidates on deploy.
    Static,

    /// Immutable data bound to a specific block (confirmed tx data, block status).
    /// Etag = {version}-{block_hash_prefix:x}. Invalidates naturally on reorg.
    BlockBound(Version, BlockHashPrefix),

    /// Mempool data — etag from next projected block hash.
    /// Etag = m{hash:x}. Invalidates on mempool change.
    MempoolHash(u64),
}

/// Resolved cache parameters
pub struct CacheParams {
    pub etag: Option<String>,
    pub cache_control: String,
}

impl CacheParams {
    pub fn immutable(version: Version) -> Self {
        Self {
            etag: Some(format!("i{version}")),
            cache_control: "public, max-age=1, must-revalidate".into(),
        }
    }

    /// Cache params using CARGO_PKG_VERSION as etag (for openapi.json etc.)
    pub fn static_version() -> Self {
        Self {
            etag: Some(format!("s{VERSION}")),
            cache_control: "public, max-age=1, must-revalidate".into(),
        }
    }

    pub fn etag_str(&self) -> &str {
        self.etag.as_deref().unwrap_or("")
    }

    pub fn matches_etag(&self, headers: &HeaderMap) -> bool {
        self.etag
            .as_ref()
            .is_some_and(|etag| headers.has_etag(etag))
    }

    pub fn resolve(strategy: &CacheStrategy, tip: impl FnOnce() -> BlockHashPrefix) -> Self {
        let cache_control = "public, max-age=1, must-revalidate".into();
        match strategy {
            CacheStrategy::Tip => Self {
                etag: Some(format!("t{:x}", *tip())),
                cache_control,
            },
            CacheStrategy::Immutable(v) => Self {
                etag: Some(format!("i{v}")),
                cache_control,
            },
            CacheStrategy::BlockBound(v, prefix) => Self {
                etag: Some(format!("b{v}-{:x}", **prefix)),
                cache_control,
            },
            CacheStrategy::Static => Self {
                etag: Some(format!("s{VERSION}")),
                cache_control,
            },
            CacheStrategy::MempoolHash(hash) => Self {
                etag: Some(format!("m{hash:x}")),
                cache_control,
            },
        }
    }
}
