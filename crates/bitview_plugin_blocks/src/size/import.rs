use bitview_collections::Windows;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyWindowStartVec, PerBlockFull, PerBlockRolling};
use brk_error::Result;
use brk_types::{Height, StoredU64, Version, Weight};
use vecdb::{CacheBudget, Database};

use super::Vecs;

fn block_vbytes(_: Height, weight: Weight) -> StoredU64 {
    StoredU64::from(weight.to_vbytes_floor())
}

impl Vecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        indexer: &Indexer,
        mappings: &MappingsVecs,
        window_starts: &Windows<&LazyWindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            vbytes: PerBlockFull::forced_import(
                cache,
                db,
                "block_vbytes",
                version,
                &indexer.vecs().blocks.weight,
                block_vbytes,
                mappings,
                window_starts,
            )?,
            size: PerBlockRolling::forced_import(
                cache,
                db,
                "block_size",
                version,
                mappings,
                window_starts,
            )?,
        })
    }
}
