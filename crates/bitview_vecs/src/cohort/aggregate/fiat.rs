use crate::{ColumnarPerBlock, FiatType, LazyFiatPerBlock};
use bitview_cohort::{UTXOAggregate, UTXOAggregateId};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, ReadableColumnarVec, Rw, StorageMode};

#[derive(Deref, DerefMut, Traversable)]
pub struct AggregateFiatPerBlock<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlock<C, UTXOAggregateId, UTXOAggregate<LazyFiatPerBlock<C>>, M>,
}

impl<C: FiatType> AggregateFiatPerBlock<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &crate::IndexSources,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_cents_by_aggregate"),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = id.metric_name(metric);
                    LazyFiatPerBlock::from_cents_source(
                        &name,
                        version,
                        &cache.wrap(source.column(&format!("{name}_cents"), version, id)),
                        indexes,
                    )
                })
            },
        )?;
        Ok(Self { values })
    }
}
