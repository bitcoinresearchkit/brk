use brk_error::Result;

use bitview_compute::{CachedWindowStartVec, PerBlockCumulativeRolling, Windows};
use brk_types::Version;
use vecdb::Database;

use super::Vecs;

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        total: PerBlockCumulativeRolling::forced_import(
            db,
            "total_sigop_cost",
            version,
            mappings,
            cached_starts,
        )?,
    })
}
