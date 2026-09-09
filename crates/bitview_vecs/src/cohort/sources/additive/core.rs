use std::ops::AddAssign;

use bitview_cohort::{
    AgeRange, ByEntry, ByEpoch, Class, CohortContext, CohortId, UTXOAggregate, UTXOCoreValues,
    UTXOGroupsWithoutAmountOrType,
};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, Database, PcoVecValue, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use crate::{StoredSeries, import_stored};

/// Independently stored cohort sources, composed from the domain's named groups.
#[derive(Traversable)]
pub struct UTXOCoreSources<T: PcoVecValue, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<StoredSeries<Height, T, M>>,
}

impl<T: PcoVecValue + AddAssign> UTXOCoreSources<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            cohorts: UTXOGroupsWithoutAmountOrType::try_new(|cohort_id| {
                import_stored(
                    cache,
                    db,
                    &CohortContext::Utxo.metric_name(cohort_id, name),
                    version + Version::TWO,
                )
            })?,
        })
    }

    pub fn get(&self, cohort_id: CohortId) -> Option<&StoredSeries<Height, T>> {
        self.cohorts.get(cohort_id)
    }

    pub fn min_len(&self) -> usize {
        self.cohorts.iter().map(AnyVec::len).min().unwrap_or(0)
    }

    pub fn push(&mut self, cohort_values: impl Into<UTXOCoreValues<T>>) {
        self.push_with_aggregate(cohort_values.into(), None);
    }

    pub(crate) fn push_with_aggregate(
        &mut self,
        cohort_values: UTXOCoreValues<T>,
        aggregate: Option<&UTXOAggregate<T>>,
    ) {
        let values = UTXOGroupsWithoutAmountOrType::new(|cohort_id| {
            aggregate
                .and_then(|values| values.get(cohort_id))
                .copied()
                .unwrap_or_else(|| cohort_values.value(cohort_id).expect("core cohort"))
        });
        for (target, &value) in self.cohorts.iter_mut().zip(values.iter()) {
            target.push(value);
        }
    }

    pub fn collect_last(&self) -> Option<UTXOCoreValues<T>> {
        Some(UTXOCoreValues {
            age_range: AgeRange::try_from_fn(|id| {
                id.select(&self.cohorts.age).collect_last().ok_or(())
            })
            .ok()?,
            epoch: ByEpoch::try_from_fn(|id| {
                id.select(&self.cohorts.epoch).collect_last().ok_or(())
            })
            .ok()?,
            class: Class::try_from_fn(|id| id.select(&self.cohorts.class).collect_last().ok_or(()))
                .ok()?,
            entry: ByEntry::try_from_fn(|id| {
                id.select(&self.cohorts.entry).collect_last().ok_or(())
            })
            .ok()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.cohorts
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .collect()
    }
}
