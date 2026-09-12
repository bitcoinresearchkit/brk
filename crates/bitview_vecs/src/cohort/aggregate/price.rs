use bitview_cohort::UTXOAggregate;
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{Database, ReadableBoxedVec, Rw};

use crate::{AggregatePerBlock, IndexSources, LazyPriceWithRatioPerBlock, import_cached};

pub type AggregatePriceWithRatioPerBlock<M = Rw> =
    AggregatePerBlock<LazyPriceWithRatioPerBlock, Cents, M>;

impl AggregatePriceWithRatioPerBlock {
    pub fn forced_import(
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &IndexSources,
        spot_price: &ReadableBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let stored = UTXOAggregate::try_from_fn(|id| {
            import_cached(
                db,
                &format!("{}_cents", id.metric_name(metric)),
                version + Version::ONE,
            )
        })?;
        let series = UTXOAggregate::from_fn(|id| {
            LazyPriceWithRatioPerBlock::from_height_source(
                &id.metric_name(metric),
                version,
                id.select(&stored),
                indexes,
                spot_price,
            )
        });
        Ok(Self { series, stored })
    }
}
