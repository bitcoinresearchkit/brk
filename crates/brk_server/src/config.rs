use std::path::PathBuf;

use brk_website::Website;

use crate::cache::CdnCacheMode;

/// Default max series-query response weight for non-loopback clients.
/// `4 * 8 * 10_000` = 320 KB (4 vecs x 8 bytes x 10k rows).
pub const DEFAULT_MAX_WEIGHT: usize = 4 * 8 * 10_000;

/// Default max series-query response weight for loopback clients.
pub const DEFAULT_MAX_WEIGHT_LOCALHOST: usize = 50 * 1_000_000;

/// Default LRU capacity for the in-process response cache.
pub const DEFAULT_CACHE_SIZE: usize = 1_000;

/// Server-wide configuration set at startup.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub data_path: PathBuf,
    pub website: Website,
    pub cdn_cache_mode: CdnCacheMode,
    pub max_weight: usize,
    pub max_weight_localhost: usize,
    pub cache_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            data_path: PathBuf::default(),
            website: Website::default(),
            cdn_cache_mode: CdnCacheMode::default(),
            max_weight: DEFAULT_MAX_WEIGHT,
            max_weight_localhost: DEFAULT_MAX_WEIGHT_LOCALHOST,
            cache_size: DEFAULT_CACHE_SIZE,
        }
    }
}
