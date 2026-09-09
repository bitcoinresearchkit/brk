use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::PerBlock;
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
) -> Result<Vecs> {
    Ok(Vecs {
        count: PerBlock::forced_import(cache, db, "utxo_count_bis", version, mappings)?,
    })
}
