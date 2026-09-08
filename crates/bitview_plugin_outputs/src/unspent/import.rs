use bitview_vecs::PerBlock;
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
) -> Result<Vecs> {
    Ok(Vecs {
        count: PerBlock::forced_import(cache, db, "utxo_count_bis", version, mappings)?,
    })
}
