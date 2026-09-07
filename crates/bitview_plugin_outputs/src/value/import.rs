use brk_error::Result;

use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use bitview_compute::ValuePerBlockCumulative;

pub fn forced_import(
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
) -> Result<Vecs> {
    Ok(Vecs {
        op_return: ValuePerBlockCumulative::forced_import(
            db,
            "op_return_value",
            version,
            mappings,
        )?,
    })
}
