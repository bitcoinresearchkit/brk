use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, LazyPerSecondWindows};
use brk_error::Result;

use super::{STORAGE, Vecs, by_type, count, spent, unspent, value};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 20_000_000)?;
        let version = STORAGE.schema_version();

        let spent = spent::forced_import(&db, version)?;
        let count = count::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let per_sec =
            LazyPerSecondWindows::new("outputs_per_sec", version, &count.total.rolling.sum);
        let unspent = unspent::forced_import(context.cache_budget(), &db, version, mappings)?;
        let by_type = by_type::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let value = value::forced_import(context.cache_budget(), &db, version, mappings)?;

        let this = Self {
            db,
            spent,
            count,
            per_sec,
            unspent,
            by_type,
            value,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
