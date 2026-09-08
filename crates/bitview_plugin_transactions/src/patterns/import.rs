use bitview_collections::Windows;
use bitview_vecs::{
    CachedWindowStartVec, ColumnarPerBlockCumulativeRolling, LazyColumnPerBlockCumulativeRolling,
};
use brk_error::Result;
use brk_types::{StoredBool, TxIndex, Version};
use vecdb::{
    CacheBudget, ColumnarVec, Database, EagerVec, ImportableVec, PcoVec, ReadOnlyClone,
    ReadableColumnarVec,
};

use super::{CountVecs, Flags, PatternId, Vecs};

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    let count_source = ColumnarPerBlockCumulativeRolling::forced_import(
        db,
        "transaction_pattern_count_cumulative",
        version,
        |_| (),
    )?;
    let counts = count_source.cumulative.read_only_clone();
    let count = CountVecs {
        coinjoin: LazyColumnPerBlockCumulativeRolling::new(
            cache,
            "coinjoin_count",
            version,
            &counts,
            PatternId::Coinjoin,
            mappings,
            cached_starts,
        ),
        consolidation: LazyColumnPerBlockCumulativeRolling::new(
            cache,
            "consolidation_count",
            version,
            &counts,
            PatternId::Consolidation,
            mappings,
            cached_starts,
        ),
        batch_payout: LazyColumnPerBlockCumulativeRolling::new(
            cache,
            "batch_payout_count",
            version,
            &counts,
            PatternId::BatchPayout,
            mappings,
            cached_starts,
        ),
        source: count_source,
    };

    let flags_source =
        EagerVec::<ColumnarVec<PcoVec<TxIndex, StoredBool>, PatternId>>::forced_import(
            db,
            "transaction_pattern_flags",
            version,
        )?;
    let flags = flags_source.read_only_clone();

    Ok(Vecs {
        count,
        flags: Flags {
            is_coinjoin: flags.column("is_coinjoin", version, PatternId::Coinjoin),
            is_consolidation: flags.column("is_consolidation", version, PatternId::Consolidation),
            is_batch_payout: flags.column("is_batch_payout", version, PatternId::BatchPayout),
        },
        flags_source,
    })
}
