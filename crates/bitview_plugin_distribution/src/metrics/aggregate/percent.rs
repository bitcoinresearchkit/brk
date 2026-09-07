use brk_error::Result;

use bitview_cohort::{
    CohortContext, UTXO_AGGREGATE_FILTERS, UTXO_AGGREGATE_NAMES, UTXOAggregate, UTXOAggregateId,
};
use bitview_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use bitview_compute::{ColumnarPerBlock, FixedRatio, LazyColumnPercentPerBlock};

#[derive(Deref, DerefMut, Traversable)]
pub struct AggregatePercentPerBlock<B: FixedRatio, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlock<
        B,
        UTXOAggregateId,
        UTXOAggregate<LazyColumnPercentPerBlock<B, UTXOAggregateId>>,
        M,
    >,
}

impl<B: FixedRatio> AggregatePercentPerBlock<B> {
    pub fn forced_import(
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_{}_by_aggregate", B::SUFFIX),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = CohortContext::Utxo.metric_name(
                        id.select(&UTXO_AGGREGATE_FILTERS),
                        id.select(&UTXO_AGGREGATE_NAMES).id,
                        metric,
                    );
                    LazyColumnPercentPerBlock::new(&name, version, source, id, mappings)
                })
            },
        )?;
        Ok(Self { values })
    }
}
