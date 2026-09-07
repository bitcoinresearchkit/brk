use brk_error::Result;

use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use bitview_compute::PerBlock;

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
) -> Result<Vecs> {
    Ok(Vecs {
        count: PerBlock::forced_import(db, "utxo_count_bis", version, mappings)?,
    })
}
