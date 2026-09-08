use bitview_collections::Windows;
use bitview_vecs::{CachedWindowStartVec, PerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        total: PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "total_sigop_cost",
            version,
            mappings,
            cached_starts,
        )?,
    })
}
