use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::CachedWindowStartVec;
use brk_error::Result;

use super::{
    STORAGE, Vecs, count, features, fees, patterns, policy, sigops, size, versions, volume,
};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        indexer: &Indexer,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 10_000_000)?;
        let version = STORAGE.schema_version();

        let count = count::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let features = features::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let size = size::forced_import(context.cache_budget(), &db, version, indexer, mappings)?;
        let fees = fees::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let patterns = patterns::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let policy = policy::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let sigops = sigops::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let versions = versions::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let volume = volume::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
            &count.total.rolling.sum,
        )?;

        let this = Self {
            db,
            count,
            features,
            size,
            fees,
            patterns,
            policy,
            sigops,
            versions,
            volume,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
