use std::ops::AddAssign;

use bitview_cohort::{
    AgeRange, ByAge, ByEntry, ByEpoch, ByTerm, Class, CohortContext, Filter, UTXOCoreValues,
    UTXOGroupCore, UTXOGroupsWithoutAmountOrType, UTXOOverlappingValues,
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
            cohorts: UTXOGroupsWithoutAmountOrType::try_new(|filter, cohort| {
                import_stored(
                    cache,
                    db,
                    &CohortContext::Utxo.metric_name(&filter, cohort, name),
                    version + Version::TWO,
                )
            })?,
        })
    }

    pub fn get(&self, filter: &Filter) -> Option<&StoredSeries<Height, T>> {
        self.cohorts.get(filter)
    }

    pub fn min_len(&self) -> usize {
        self.cohorts.iter().map(AnyVec::len).min().unwrap_or(0)
    }

    pub fn push(&mut self, cohort_values: impl Into<UTXOCoreValues<T>>) {
        self.push_with_overlapping(cohort_values.into(), None);
    }

    pub(crate) fn push_with_overlapping(
        &mut self,
        cohort_values: UTXOCoreValues<T>,
        overlapping: Option<&UTXOOverlappingValues<T>>,
    ) {
        let values = if let Some(overlapping) = overlapping {
            UTXOGroupsWithoutAmountOrType {
                core: UTXOGroupCore {
                    all: overlapping.aggregate.all,
                    age: ByAge {
                        range: cohort_values.age_range,
                        under: overlapping.under_age.clone(),
                        over: overlapping.over_age.clone(),
                    },
                    epoch: cohort_values.epoch,
                    class: cohort_values.class,
                    entry: cohort_values.entry,
                },
                term: ByTerm {
                    short: overlapping.aggregate.sth,
                    long: overlapping.aggregate.lth,
                },
            }
        } else {
            UTXOGroupsWithoutAmountOrType::new(|filter, _| {
                cohort_values.value(&filter).expect("core cohort")
            })
        };
        for (target, &value) in self.cohorts.iter_mut().zip(values.iter()) {
            target.push(value);
        }
    }

    pub fn collect_last(&self) -> Option<UTXOCoreValues<T>> {
        Some(UTXOCoreValues {
            age_range: AgeRange::try_from_fn(|id| {
                id.select(&self.cohorts.age.range).collect_last().ok_or(())
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
