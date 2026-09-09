use std::ops::AddAssign;

use bitview_cohort::{
    Amount, AmountRange, AmountRangeId, CohortContext, Filter, SpendableType,
    UTXOOverlappingValues, UTXOValues,
};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, Database, PcoVecValue, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use super::UTXOTypedSources;
use crate::{StoredSeries, import_stored};

#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOSources<T: PcoVecValue, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub typed: UTXOTypedSources<T, M>,
    pub amount: Amount<StoredSeries<Height, T, M>>,
}

impl<T: PcoVecValue + AddAssign> UTXOSources<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            typed: UTXOTypedSources::forced_import(cache, db, name, version)?,
            amount: Amount::try_new(|filter, cohort| {
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
        match filter {
            Filter::Amount(_) => self.amount.get(filter),
            _ => self.typed.get(filter),
        }
    }

    pub fn min_len(&self) -> usize {
        self.amount
            .iter()
            .map(AnyVec::len)
            .fold(self.typed.min_len(), usize::min)
    }

    pub fn push(&mut self, cohort_values: UTXOValues<T>) {
        self.push_with_overlapping(cohort_values, None);
    }

    pub fn push_exact(
        &mut self,
        cohort_values: UTXOValues<T>,
        overlapping: UTXOOverlappingValues<T>,
    ) {
        self.push_with_overlapping(cohort_values, Some(&overlapping));
    }

    fn push_with_overlapping(
        &mut self,
        cohort_values: UTXOValues<T>,
        overlapping: Option<&UTXOOverlappingValues<T>>,
    ) {
        let values = if let Some(overlapping) = overlapping {
            Amount {
                range: cohort_values.amount_range,
                under: overlapping.under_amount.clone(),
                over: overlapping.over_amount.clone(),
            }
        } else {
            Amount::new(|filter, _| {
                if let Some(id) = AmountRangeId::matching(&filter) {
                    return *id.select(&cohort_values.amount_range);
                }
                let mut ranges = AmountRangeId::included_by(&filter);
                let mut total = *ranges
                    .next()
                    .expect("nonempty amount cohort")
                    .select(&cohort_values.amount_range);
                for id in ranges {
                    total += *id.select(&cohort_values.amount_range);
                }
                total
            })
        };
        for (target, &value) in self.amount.iter_mut().zip(values.iter()) {
            target.push(value);
        }
        self.typed
            .push_with_overlapping(cohort_values.core, cohort_values.type_, overlapping);
    }

    pub fn collect_last(&self) -> Option<UTXOValues<T>> {
        Some(UTXOValues {
            amount_range: AmountRange::try_from_fn(|id| {
                id.select(&self.amount.range).collect_last().ok_or(())
            })
            .ok()?,
            type_: SpendableType::try_new(|filter, _| {
                let Filter::Type(output_type) = filter else {
                    unreachable!()
                };
                self.type_.get(output_type).collect_last().ok_or(())
            })
            .ok()?,
            core: self.typed.core.collect_last()?,
        })
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.typed.collect_vecs_mut();
        vecs.extend(self.amount.iter_mut().map(|v| v as &mut dyn AnyStoredVec));
        vecs
    }
}
