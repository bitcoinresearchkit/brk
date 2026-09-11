use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{
    LazyPerSecondWindows, LazyRollingSumsFromHeight, LazyWindowStartVec,
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
    mappings: &MappingsVecs,
    window_starts: &Windows<&LazyWindowStartVec>,
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
            window_starts,
        )?,
        tx_per_sec: LazyPerSecondWindows::new("tx_per_sec", v, tx_count_sums),
    })
}
