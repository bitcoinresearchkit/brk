use bitview_collections::Windows;
use bitview_vecs::{
    CachedWindowStartVec, LazyPerSecondWindows, LazyRollingSumsFromHeight,
    ValuePerBlockCumulativeRolling,
};
use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::{CacheBudget, Database};

use super::Vecs;

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
    tx_count_sums: &LazyRollingSumsFromHeight<StoredU64>,
) -> Result<Vecs> {
    let v = version + Version::TWO;
    Ok(Vecs {
        transfer_volume: ValuePerBlockCumulativeRolling::forced_import(
            cache,
            db,
            "transfer_volume_bis",
            version,
            mappings,
            cached_starts,
        )?,
        tx_per_sec: LazyPerSecondWindows::new("tx_per_sec", v, tx_count_sums),
    })
}
