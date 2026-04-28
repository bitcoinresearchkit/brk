use std::path::PathBuf;

use brk_website::Website;

use crate::cache::CdnCacheMode;

/// Default max series-query response weight.
/// 50 MB - generous enough for any honest query, low enough to limit cache-buster leverage.
pub const DEFAULT_MAX_WEIGHT: usize = 50 * 1_000_000;

/// Server-wide configuration set at startup.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub data_path: PathBuf,
    pub website: Website,
    pub cdn_cache_mode: CdnCacheMode,
    pub max_weight: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            data_path: PathBuf::default(),
            website: Website::default(),
            cdn_cache_mode: CdnCacheMode::default(),
            max_weight: DEFAULT_MAX_WEIGHT,
        }
    }
}
