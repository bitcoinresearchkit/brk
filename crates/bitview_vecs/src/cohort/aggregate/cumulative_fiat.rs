use bitview_cohort::UTXOAggregate;
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, Database, ReadableCloneableVec, ReadableVec, Rw,
    StorageMode, WritableVec,
};

use crate::{
    CumulativeState, FiatType, IndexSources, LazyFiatPerBlockCumulativeWithSums, StoredSeries,
    import_stored,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AdditiveAggregateFiatPerBlockCumulativeWithSums<C: FiatType, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub series: UTXOAggregate<LazyFiatPerBlockCumulativeWithSums<C>>,
    #[traversable(hidden)]
    pub stored: UTXOAggregate<StoredSeries<Height, C, M>>,
    last: M::WriteOnly<CumulativeState<UTXOAggregate<C>>>,
}

impl<C: FiatType> AdditiveAggregateFiatPerBlockCumulativeWithSums<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        indexes: &IndexSources,
        cached_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let stored = UTXOAggregate::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{}_cumulative_cents", id.metric_name(metric)),
                version + Version::ONE,
            )
        })?;
        let series = UTXOAggregate::from_fn(|id| {
            LazyFiatPerBlockCumulativeWithSums::from_cumulative_cents_source(
                &id.metric_name(metric),
                version,
                id.select(&stored),
                indexes,
                cached_starts,
            )
        });
        Ok(Self {
            series,
            stored,
            last: Default::default(),
        })
    }

    pub fn push_block(&mut self, mut values: UTXOAggregate<C>) {
        values.all = values.sth + values.lth;
        let len = self.len();
        let cumulative = self.last.accumulate(
            len,
            || {
                UTXOAggregate::try_from_fn(|id| id.select(&self.stored).collect_last().ok_or(()))
                    .ok()
            },
            |last| {
                for (last, &value) in last.iter_mut().zip(values.iter()) {
                    *last += value;
                }
            },
        );
        for (target, &value) in self.stored.iter_mut().zip(cumulative.iter()) {
            target.push(value);
        }
    }

    pub fn len(&self) -> usize {
        self.stored.iter().map(AnyVec::len).min().unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.last = Default::default();
        self.stored
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .collect()
    }
}
