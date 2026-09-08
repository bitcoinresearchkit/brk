use bitview_vecs::ValuePerBlockCumulative;
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
        op_return: ValuePerBlockCumulative::forced_import(
            cache,
            db,
            "op_return_value",
            version,
            mappings,
        )?,
    })
}
