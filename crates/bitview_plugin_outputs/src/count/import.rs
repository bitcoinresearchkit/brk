use brk_error::Result;

use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use bitview_compute::{CachedWindowStartVec, PerBlockAggregated, Windows};

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        total: PerBlockAggregated::forced_import(
            db,
            "output_count",
            version,
            mappings.output_count_source(),
            mappings,
            cached_starts,
        )?,
    })
}
