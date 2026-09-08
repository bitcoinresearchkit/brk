use bitview_collections::Windows;
use bitview_vecs::{CachedWindowStartVec, PerBlockCumulativeAverage};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

impl Vecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let interval = PerBlockCumulativeAverage::forced_import(
            cache,
            db,
            "block_interval",
            version,
            mappings,
            cached_starts,
        )?;

        Ok(Self(interval))
    }
}
