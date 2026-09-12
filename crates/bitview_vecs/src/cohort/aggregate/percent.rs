use bitview_cohort::UTXOAggregate;
use bitview_compute::FixedRatio;
use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, Rw};

use crate::{AggregatePerBlock, IndexSources, LazyPercentPerBlock, import_cached};

pub type AggregatePercentPerBlock<B, M = Rw> = AggregatePerBlock<LazyPercentPerBlock<B>, B, M>;

impl<B: FixedRatio> AggregatePercentPerBlock<B> {
    pub fn forced_import(
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let stored = UTXOAggregate::try_from_fn(|id| {
            import_cached(
                db,
                &format!("{}_{}", id.metric_name(metric), B::SUFFIX),
                version + Version::ONE,
            )
        })?;
        let series = UTXOAggregate::from_fn(|id| {
            LazyPercentPerBlock::from_height_source(
                &id.metric_name(metric),
                version,
                id.select(&stored),
                indexes,
            )
        });
        Ok(Self { series, stored })
    }
}
