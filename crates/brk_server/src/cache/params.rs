use axum::http::HeaderMap;
use brk_types::{BlockHashPrefix, Version};

use crate::{VERSION, etag::Etag, extended::HeaderMapExtended};

use super::{
    mode::{CDN_LIVE, cdn_cached},
    strategy::CacheStrategy,
};

// Browser-facing: always revalidate via ETag. `no-cache` means "cache it but
// check before use" (not "don't cache"); ETag makes the check cheap.
const CC: &str = "public, no-cache, stale-if-error=86400";

// Errors: short, must-revalidate, no `stale-if-error` (we don't want a 24h-old
// error served when origin recovers). Same string for browser and CDN.
pub(crate) const CC_ERROR: &str = "public, max-age=1, must-revalidate";

/// Resolved cache parameters: an ETag plus the two Cache-Control directives.
pub struct CacheParams {
    pub etag: Etag,
    cache_control: &'static str,
    cdn_cache_control: &'static str,
}

impl CacheParams {
    fn tip(tip: BlockHashPrefix) -> Self {
        Self {
            etag: format!("t{:x}", *tip).into(),
            cache_control: CC,
            cdn_cache_control: CDN_LIVE,
        }
    }

    fn immutable(version: Version) -> Self {
        Self {
            etag: format!("i{version}").into(),
            cache_control: CC,
            cdn_cache_control: cdn_cached(),
        }
    }

    fn block_bound(version: Version, prefix: BlockHashPrefix) -> Self {
        Self {
            etag: format!("b{version}-{:x}", *prefix).into(),
            cache_control: CC,
            cdn_cache_control: cdn_cached(),
        }
    }

    /// Deploy-tied response: etag from the build version. Used directly
    /// by static handlers (OpenAPI spec, scalar bundle) that don't have
    /// a [`CacheStrategy`] context.
    pub fn deploy() -> Self {
        Self {
            etag: format!("d{VERSION}").into(),
            cache_control: CC,
            cdn_cache_control: cdn_cached(),
        }
    }

    fn mempool_hash(hash: u64) -> Self {
        Self {
            etag: format!("m{hash:x}").into(),
            cache_control: CC,
            cdn_cache_control: CDN_LIVE,
        }
    }

    /// Series query: tail-bound (`end >= total`) gets LIVE, historical gets CACHED.
    /// Etag distinguishes the two: tail uses tip hash (per-block + reorgs),
    /// historical uses total length (only changes when new data is appended).
    pub fn series(version: Version, total: usize, end: usize, hash: BlockHashPrefix) -> Self {
        let v = u32::from(version);
        if end >= total {
            Self {
                etag: format!("s{v}-{:x}", *hash).into(),
                cache_control: CC,
                cdn_cache_control: CDN_LIVE,
            }
        } else {
            Self {
                etag: format!("s{v}-{total}").into(),
                cache_control: CC,
                cdn_cache_control: cdn_cached(),
            }
        }
    }

    /// Error response: keeps the originating ETag (so retries can 304),
    /// uses [`CC_ERROR`] for both browser and CDN.
    pub fn error(etag: Etag) -> Self {
        Self {
            etag,
            cache_control: CC_ERROR,
            cdn_cache_control: CC_ERROR,
        }
    }

    pub fn matches_etag(&self, headers: &HeaderMap) -> bool {
        headers.has_etag(self.etag.as_str())
    }

    /// Write this cache policy (etag + cache-control + cdn-cache-control) onto a response's headers.
    pub fn apply_to(&self, headers: &mut HeaderMap) {
        headers.insert_etag(self.etag.as_str());
        headers.insert_cache_control(self.cache_control);
        headers.insert_cdn_cache_control(self.cdn_cache_control);
    }

    pub fn resolve(strategy: &CacheStrategy, tip: BlockHashPrefix) -> Self {
        match strategy {
            CacheStrategy::Tip => Self::tip(tip),
            CacheStrategy::Immutable(v) => Self::immutable(*v),
            CacheStrategy::BlockBound(v, prefix) => Self::block_bound(*v, *prefix),
            CacheStrategy::Deploy => Self::deploy(),
            CacheStrategy::MempoolHash(hash) => Self::mempool_hash(*hash),
        }
    }
}
