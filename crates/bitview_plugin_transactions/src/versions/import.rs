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
    let import = |name| {
        PerBlockCumulativeRolling::forced_import(
            cache,
            db,
            name,
            version + Version::ONE,
            mappings,
            cached_starts,
        )
    };
    Ok(Vecs {
        v1: import("tx_v1")?,
        v2: import("tx_v2")?,
        v3: import("tx_v3")?,
        other: import("tx_other_version")?,
    })
}
