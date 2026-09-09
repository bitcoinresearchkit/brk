use bitview_cohort::UTXOAggregate;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, CacheBudget, Database, Rw, StorageMode, WritableVec};

use crate::{FiatType, IndexSources, LazyFiatPerBlock, StoredSeries, import_stored};

#[derive(Deref, DerefMut, Traversable)]
pub struct AggregateFiatPerBlock<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub series: UTXOAggregate<LazyFiatPerBlock<C>>,
    #[traversable(hidden)]
    pub stored: UTXOAggregate<StoredSeries<Height, C, M>>,
}

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

    pub fn push(&mut self, values: UTXOAggregate<C>) {
        for (target, &value) in self.stored.iter_mut().zip(values.iter()) {
            target.push(value);
        }
    }

    pub fn push_additive(&mut self, mut values: UTXOAggregate<C>) {
        values.all = values.sth + values.lth;
        self.push(values);
    }

    pub fn len(&self) -> usize {
        self.stored.iter().map(AnyVec::len).min().unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.stored
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .collect()
    }
}
