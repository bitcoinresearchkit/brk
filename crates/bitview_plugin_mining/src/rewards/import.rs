use bitview_plugin_mappings::Vecs as MappingsVecs;
use brk_error::Result;

use bitview_compute::{
    CachedWindowStartVec, LazyPercentCumulativeRolling, LazyPercentRollingWindows, OneMinusPpm,
    RatioSats, ValuePerBlockCumulative, ValuePerBlockCumulativeRolling, ValuePerBlockFull, Windows,
};
use bitview_plugin_indexer::Indexer;
use brk_types::{PartsPerMillion32, PartsPerMillion64, Sats, Version};
use vecdb::{AnyVec, Database, EagerVec, ImportableVec};

use super::Vecs;

pub fn forced_import(
    db: &Database,
    version: Version,
    indexer: &Indexer,
    mappings: &MappingsVecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
) -> Result<Vecs> {
    let coinbase_version = version
        + indexer.vecs().transactions.first_txout_index.version()
        + mappings.tx_index.output_count.version()
        + indexer.vecs().outputs.value.version();

    let coinbase = ValuePerBlockCumulativeRolling::forced_import(
        db,
        "coinbase",
        coinbase_version,
        mappings,
        cached_starts,
    )?;
    let subsidy = ValuePerBlockCumulativeRolling::forced_import(
        db,
        "subsidy",
        version,
        mappings,
        cached_starts,
    )?;
    let fees = ValuePerBlockFull::forced_import(db, "fees", version, mappings, cached_starts)?;
    let fees_source = fees.cumulative_sats_source();

    let fee_dominance = LazyPercentCumulativeRolling::from_cumulative_ratio_with_numerator::<
        Sats,
        Sats,
        RatioSats<PartsPerMillion32>,
    >(
        "fee_dominance",
        version,
        fees_source,
        coinbase.cumulative.sats.resolutions.height_source(),
        cached_starts,
        mappings,
    );
    let subsidy_dominance = LazyPercentCumulativeRolling::from_lazy_source::<OneMinusPpm>(
        "subsidy_dominance",
        version,
        &fee_dominance,
    );
    let fee_to_subsidy = LazyPercentRollingWindows::from_cumulative_ratio_with_numerator::<
        Sats,
        Sats,
        RatioSats<PartsPerMillion64>,
    >(
        "fee_to_subsidy",
        version + Version::ONE,
        fees_source,
        subsidy.cumulative.sats.resolutions.height_source(),
        cached_starts,
        mappings,
    );

    Ok(Vecs {
        coinbase,
        subsidy,
        fees,
        output_volume: EagerVec::forced_import(db, "output_volume", version)?,
        unclaimed: ValuePerBlockCumulative::forced_import(
            db,
            "unclaimed_rewards",
            version,
            mappings,
        )?,
        fee_dominance,
        subsidy_dominance,
        fee_to_subsidy,
    })
}
