use bitview_cohort::{CohortContext, UTXO_AGGREGATE_FILTERS, UTXO_AGGREGATE_NAMES, UTXOAggregate};
use bitview_compute::FixedRatio;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{LazyPercentPerBlock, StoredSeries};
use brk_types::{Height, PartsPerMillion32, Version};
use vecdb::LazyVec;

mod gross_pnl_composition;
mod source;
mod supply_profitability_shares;
mod vecs;

pub use gross_pnl_composition::GrossPnlComposition;
pub use source::RelativeSource;
pub use supply_profitability_shares::SupplyProfitabilityShares;
pub use vecs::RelativeVecs;

fn share_views<B: FixedRatio>(
    sources: &UTXOAggregate<StoredSeries<Height, PartsPerMillion32>>,
    metric: &str,
    version: Version,
    compute: fn(Height, PartsPerMillion32) -> B,
    mappings: &MappingsVecs,
) -> UTXOAggregate<LazyPercentPerBlock<B>> {
    UTXOAggregate::from_fn(|id| {
        let name = CohortContext::Utxo.metric_name(
            id.select(&UTXO_AGGREGATE_FILTERS),
            id.select(&UTXO_AGGREGATE_NAMES).id,
            metric,
        );
        let source = id.select(sources);
        let source = LazyVec::init(
            &format!("{name}_{}_source", B::SUFFIX),
            version,
            source.read_only_boxed_clone(),
            compute,
        );
        LazyPercentPerBlock::from_height_source(&name, version, &source, mappings)
    })
}
