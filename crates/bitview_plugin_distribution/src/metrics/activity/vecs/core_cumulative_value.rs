use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType, UTXORows};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyValuePerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{Cents, Sats, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOValueColumnarMetricWithoutAmountOrType;

#[derive(Traversable)]
pub struct CoreCumulativeValueByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyValuePerBlockCumulativeRolling>,
    pub cumulative: CumulativeUTXOValueColumnarMetricWithoutAmountOrType<M>,
}

impl CoreCumulativeValueByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let cumulative = CumulativeUTXOValueColumnarMetricWithoutAmountOrType::forced_import(
            db,
            &format!("{metric}_cumulative"),
            version,
        )?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let (sats, cents) = cumulative
                .sources(cache, &filter, &name, version)
                .expect("supported core cumulative value cohort");
            LazyValuePerBlockCumulativeRolling::from_cumulative_sources(
                &name,
                version,
                &sats,
                &cents,
                mappings,
                cached_starts,
            )
        });
        Ok(Self {
            cohorts,
            cumulative,
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, sats: UTXORows<Sats>, cents: UTXORows<Cents>) {
        self.cumulative.push_block(sats, cents);
    }

    pub fn min_len(&self) -> usize {
        self.cumulative.min_len()
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.cumulative.collect_vecs_mut()
    }
}
