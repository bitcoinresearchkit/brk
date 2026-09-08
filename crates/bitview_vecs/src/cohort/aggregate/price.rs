use crate::{ColumnarPerBlock, LazyColumnPriceWithRatioPerBlock};
use bitview_cohort::{UTXOAggregate, UTXOAggregateId};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, CachedBoxedVec, Database, Rw, StorageMode};

#[derive(Deref, DerefMut, Traversable)]
pub struct AggregatePriceWithRatioPerBlock<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub values: ColumnarPerBlock<
        Cents,
        UTXOAggregateId,
        UTXOAggregate<LazyColumnPriceWithRatioPerBlock<UTXOAggregateId>>,
        M,
    >,
}

impl AggregatePriceWithRatioPerBlock {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &crate::IndexSources,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_cents_by_aggregate"),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = id.metric_name(metric);
                    LazyColumnPriceWithRatioPerBlock::new(
                        cache, &name, version, source, id, indexes, spot_price,
                    )
                })
            },
        )?;
        Ok(Self { values })
    }
}
