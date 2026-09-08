use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyFiatPerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{Cents, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOCoreColumns;

#[derive(Traversable)]
pub struct CumulativeValueDestroyedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyFiatPerBlockCumulativeRolling<Cents>>,
    pub stored: CumulativeUTXOCoreColumns<Cents, M>,
}

impl CumulativeValueDestroyedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let metric = "value_destroyed";
        let stored = CumulativeUTXOCoreColumns::forced_import(
            db,
            "value_destroyed_cumulative_cents",
            version,
        )?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let source = stored
                .columns
                .additive_source(cache, &filter, &format!("{name}_cumulative_cents"), version)
                .expect("supported value-destroyed cohort");
            LazyFiatPerBlockCumulativeRolling::from_cumulative_cents_source(
                &name,
                version,
                &source,
                mappings,
                cached_starts,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
