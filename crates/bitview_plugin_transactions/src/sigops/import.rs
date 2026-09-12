use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyWindowStartVec, PerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    window_starts: &Windows<&LazyWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        total: PerBlockCumulativeRolling::forced_import(
            db,
            "total_sigop_cost",
            version,
            mappings,
            window_starts,
        )?,
    })
}
