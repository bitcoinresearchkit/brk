use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyWindowStartVec, PerBlockCumulativeRolling, import_stored};
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::{CountVecs, Flags, Vecs};

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    window_starts: &Windows<&LazyWindowStartVec>,
) -> Result<Vecs> {
    let version = version + Version::ONE;
    let count = |name| {
        PerBlockCumulativeRolling::forced_import(cache, db, name, version, mappings, window_starts)
    };
    Ok(Vecs {
        count: CountVecs {
            coinjoin: count("coinjoin_count")?,
            consolidation: count("consolidation_count")?,
            batch_payout: count("batch_payout_count")?,
        },
        flags: Flags {
            is_coinjoin: import_stored(cache, db, "is_coinjoin", version)?,
            is_consolidation: import_stored(cache, db, "is_consolidation", version)?,
            is_batch_payout: import_stored(cache, db, "is_batch_payout", version)?,
        },
    })
}
