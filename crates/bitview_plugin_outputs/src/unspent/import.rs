use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::PerBlock;
use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;

pub fn forced_import(db: &Database, version: Version, mappings: &MappingsVecs) -> Result<Vecs> {
    Ok(Vecs {
        count: PerBlock::forced_import(db, "utxo_count_bis", version, mappings)?,
    })
}
