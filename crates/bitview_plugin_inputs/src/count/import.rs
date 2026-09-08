use bitview_collections::Windows;
use bitview_vecs::{CachedWindowStartVec, PerBlockAggregated};
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
        Ok(Self(PerBlockAggregated::forced_import(
            cache,
            db,
            "input_count",
            version,
            &mappings.input_count_source(),
            mappings,
            cached_starts,
        )?))
    }
}
