use bitview_plugin_mappings::Vecs as MappingsVecs;
use brk_error::Result;

use bitview_cohort::{
    CohortContext, UTXO_AGGREGATE_FILTERS, UTXO_AGGREGATE_NAMES, UTXOAggregate, UTXOAggregateId,
};
use bitview_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{AnyVec, Database, ReadableColumnarVec, Rw, StorageMode};

use bitview_compute::{CACHE_BUDGET, ColumnarPerBlock, FiatType, LazyFiatPerBlock};

#[derive(Deref, DerefMut, Traversable)]
pub struct AggregateFiatPerBlock<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlock<C, UTXOAggregateId, UTXOAggregate<LazyFiatPerBlock<C>>, M>,
}

impl<C: FiatType> AggregateFiatPerBlock<C> {
    pub fn forced_import(
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &MappingsVecs,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_cents_by_aggregate"),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = CohortContext::Utxo.metric_name(
                        id.select(&UTXO_AGGREGATE_FILTERS),
                        id.select(&UTXO_AGGREGATE_NAMES).id,
                        metric,
                    );
                    LazyFiatPerBlock::from_cents_source(
                        &name,
                        version,
                        &CACHE_BUDGET.wrap(source.column(&format!("{name}_cents"), version, id)),
                        mappings,
                    )
                })
            },
        )?;
        Ok(Self { values })
    }

    pub fn len(&self) -> usize {
        self.values.height.len()
    }
}
