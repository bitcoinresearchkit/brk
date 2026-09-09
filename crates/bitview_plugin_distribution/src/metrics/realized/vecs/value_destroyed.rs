use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyFiatPerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{Cents, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOCoreSources;

#[derive(Traversable)]
pub struct CumulativeValueDestroyedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyFiatPerBlockCumulativeRolling<Cents>>,
    #[traversable(hidden)]
    pub stored: CumulativeUTXOCoreSources<Cents, M>,
}

impl CumulativeValueDestroyedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let metric = "value_destroyed";
        let stored = CumulativeUTXOCoreSources::forced_import(
            cache,
            db,
            "value_destroyed_cumulative_cents",
            version,
        )?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, metric);
            let source = stored
                .stored
                .get(&filter)
                .expect("supported value-destroyed cohort");
            LazyFiatPerBlockCumulativeRolling::from_cumulative_cents_source(
                &name,
                version,
                source,
                mappings,
                cached_starts,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
