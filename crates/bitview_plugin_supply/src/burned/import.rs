use bitview_vecs::ValuePerBlockCumulative;
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database};

use super::Vecs;

impl Vecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            total: ValuePerBlockCumulative::forced_import(
                cache,
                db,
                "unspendable_supply",
                version,
                mappings,
            )?,
        })
    }
}
