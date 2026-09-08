use bitview_plugin_mappings::Vecs as MappingsVecs;
mod gross_pnl_composition;
mod source;
mod supply_profitability_shares;
mod vecs;

pub use gross_pnl_composition::GrossPnlComposition;
pub use source::RelativeSource;
pub use supply_profitability_shares::SupplyProfitabilityShares;
pub use vecs::RelativeVecs;

use bitview_cohort::{
    CohortContext, UTXO_AGGREGATE_FILTERS, UTXO_AGGREGATE_NAMES, UTXOAggregate, UTXOAggregateId,
};
use bitview_compute::{CACHE_BUDGET, FixedRatio, LazyPercentPerBlock};
use brk_types::{Height, PartsPerMillion32, Version};
use vecdb::{LazyVec, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec, ReadableColumnarVec};

fn share_views<B: FixedRatio>(
    source: &ReadOnlyColumnarVec<PcoVec<Height, PartsPerMillion32>, UTXOAggregateId>,
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
        let source = CACHE_BUDGET.wrap(source.column(&format!("{name}_source"), version, id));
        let source = LazyVec::init(
            &format!("{name}_{}_source", B::SUFFIX),
            version,
            source.read_only_boxed_clone(),
            compute,
        );
        LazyPercentPerBlock::from_height_source(&name, version, &source, mappings)
    })
}
