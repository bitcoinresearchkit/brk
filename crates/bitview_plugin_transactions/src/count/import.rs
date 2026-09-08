use brk_error::Result;

use bitview_compute::{CachedWindowStartVec, PerBlockFullFromCumulative, Windows};
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
        total: PerBlockFullFromCumulative::forced_import(
            db,
            "tx_count",
            version,
            &mappings.transaction_count_source(),
            mappings,
            cached_starts,
        )?,
    })
}
