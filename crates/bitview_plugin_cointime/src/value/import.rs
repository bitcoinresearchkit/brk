use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, PerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        destroyed: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "cointime_value_destroyed",
            version,
            mappings,
            cached_starts,
        )?,
        created: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "cointime_value_created",
            version,
            mappings,
            cached_starts,
        )?,
        stored: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "cointime_value_stored",
            version,
            mappings,
            cached_starts,
        )?,
        vocdd: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "vocdd",
            version + Version::ONE,
            mappings,
            cached_starts,
        )?,
    })
}
