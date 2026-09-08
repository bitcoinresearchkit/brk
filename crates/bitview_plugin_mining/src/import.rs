use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use bitview_vecs::CachedWindowStartVec;
use brk_error::Result;

use super::{STORAGE, Vecs};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        indexer: &Indexer,
        mappings: &bitview_plugin_mappings::Vecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 1_000_000)?;
        let version = STORAGE.schema_version();

        let rewards = super::rewards::forced_import(
            context.cache_budget(),
            &db,
            version,
            indexer,
            mappings,
            cached_starts,
        )?;
        let hashrate =
            super::hashrate::forced_import(context.cache_budget(), &db, version, mappings)?;

        let this = Self {
            db,
            rewards,
            hashrate,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
