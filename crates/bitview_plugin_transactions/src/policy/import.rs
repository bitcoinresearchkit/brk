use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyWindowStartVec, PerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database, EagerVec, ImportableVec};

use super::{CountVecs, Vecs};

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    window_starts: &Windows<&LazyWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        count: CountVecs {
            nonstandard: PerBlockCumulativeRolling::forced_import(
                cache,
                db,
                "nonstandard_count",
                version,
                mappings,
                window_starts,
            )?,
        },
        is_nonstandard: EagerVec::forced_import(db, "is_nonstandard", version)?,
    })
}
