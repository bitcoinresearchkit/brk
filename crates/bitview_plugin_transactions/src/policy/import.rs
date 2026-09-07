use brk_error::Result;

use bitview_compute::{CachedWindowStartVec, PerBlockCumulativeRolling, Windows};
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::{CountVecs, Vecs};

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    Ok(Vecs {
        count: CountVecs {
            nonstandard: PerBlockCumulativeRolling::forced_import(
                db,
                "nonstandard_count",
                version,
                mappings,
                cached_starts,
            )?,
        },
        is_nonstandard: EagerVec::forced_import(db, "is_nonstandard", version)?,
    })
}
