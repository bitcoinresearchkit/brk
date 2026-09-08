use crate::{ColumnarPerBlock, LazyColumnPercentPerBlock};
use bitview_cohort::{UTXOAggregate, UTXOAggregateId};
use bitview_compute::FixedRatio;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

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
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &crate::IndexSources,
    ) -> Result<Self> {
        let values = ColumnarPerBlock::forced_import(
            db,
            &format!("{metric}_{}_by_aggregate", B::SUFFIX),
            version,
            |source| {
                UTXOAggregate::from_fn(|id| {
                    let name = id.metric_name(metric);
                    LazyColumnPercentPerBlock::new(cache, &name, version, source, id, indexes)
                })
            },
        )?;
        Ok(Self { values })
    }
}
