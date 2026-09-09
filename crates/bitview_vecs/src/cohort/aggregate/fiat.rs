use bitview_cohort::UTXOAggregate;
use brk_error::Result;
use brk_types::Version;
use vecdb::{CacheBudget, Database, Rw};

use crate::{AggregatePerBlock, FiatType, IndexSources, LazyFiatPerBlock, import_stored};

pub type AggregateFiatPerBlock<C, M = Rw> = AggregatePerBlock<LazyFiatPerBlock<C>, C, M>;

impl<C: FiatType> AggregateFiatPerBlock<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let stored = UTXOAggregate::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{}_cents", id.metric_name(metric)),
                version + Version::ONE,
            )
        })?;
        let series = UTXOAggregate::from_fn(|id| {
            LazyFiatPerBlock::from_cents_source(
                &id.metric_name(metric),
                version,
                id.select(&stored),
                indexes,
            )
        });
        Ok(Self { series, stored })
    }

    pub fn push_additive(&mut self, mut values: UTXOAggregate<C>) {
        values.all = values.sth + values.lth;
        self.push(values);
    }
}
