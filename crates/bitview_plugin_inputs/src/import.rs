use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, LazyPerSecondWindows};
use brk_error::Result;
use vecdb::{ImportableVec, PcoVec};

use super::{ByTypeVecs, CountVecs, STORAGE, Vecs};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 20_000_000)?;
        let version = STORAGE.schema_version();

        let value = PcoVec::forced_import(&db, "value", version)?;
        let count = CountVecs::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;
        let per_sec = LazyPerSecondWindows::new("inputs_per_sec", version, &count.rolling.sum);
        let by_type = ByTypeVecs::forced_import(
            context.cache_budget(),
            &db,
            version,
            mappings,
            cached_starts,
        )?;

        let this = Self {
            db,
            value,
            count,
            per_sec,
            by_type,
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
