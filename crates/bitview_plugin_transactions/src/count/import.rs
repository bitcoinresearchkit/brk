use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, PerBlockFullFromCumulative};
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
        total: PerBlockFullFromCumulative::forced_import(
            cache,
            db,
            "tx_count",
            version,
            &mappings.transaction_count_source(),
            mappings,
            cached_starts,
        )?,
    })
}
