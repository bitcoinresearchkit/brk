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

pub(crate) const CACHE_CONTROL: &str = "public, max-age=1, must-revalidate";

/// Resolved cache parameters
pub struct CacheParams {
    pub etag: Option<String>,
    pub cache_control: &'static str,
}

impl CacheParams {
    pub fn tip(tip: BlockHashPrefix) -> Self {
        Self {
            etag: Some(format!("t{:x}", *tip)),
            cache_control: CACHE_CONTROL,
        }
    }

    pub fn immutable(version: Version) -> Self {
        Self {
            etag: Some(format!("i{version}")),
            cache_control: CACHE_CONTROL,
        }
    }

    pub fn block_bound(version: Version, prefix: BlockHashPrefix) -> Self {
        Self {
            etag: Some(format!("b{version}-{:x}", *prefix)),
            cache_control: CACHE_CONTROL,
        }
    }

    pub fn static_version() -> Self {
        Self {
            etag: Some(format!("s{VERSION}")),
            cache_control: CACHE_CONTROL,
        }
    }

    pub fn mempool_hash(hash: u64) -> Self {
        Self {
            etag: Some(format!("m{hash:x}")),
            cache_control: CACHE_CONTROL,
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
        match strategy {
            CacheStrategy::Tip => Self::tip(tip()),
            CacheStrategy::Immutable(v) => Self::immutable(*v),
            CacheStrategy::BlockBound(v, prefix) => Self::block_bound(*v, *prefix),
            CacheStrategy::Static => Self::static_version(),
            CacheStrategy::MempoolHash(hash) => Self::mempool_hash(*hash),
        }
    }
}
