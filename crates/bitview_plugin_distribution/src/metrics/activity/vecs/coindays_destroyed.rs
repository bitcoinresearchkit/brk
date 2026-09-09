use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmountOrType};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyPerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{StoredF64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOCoreSources;

#[derive(Traversable)]
pub struct CoindaysDestroyedByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmountOrType<LazyPerBlockCumulativeRolling<StoredF64>>,
    #[traversable(hidden)]
    pub stored: CumulativeUTXOCoreSources<StoredF64, M>,
}

impl CoindaysDestroyedByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let stored = CumulativeUTXOCoreSources::forced_import(
            cache,
            db,
            "coindays_destroyed_cumulative",
            version,
        )?;
        let cohorts = UTXOGroupsWithoutAmountOrType::new(|cohort_id| {
            let name = CohortContext::Utxo.metric_name(cohort_id, "coindays_destroyed");
            let source = stored
                .stored
                .get(cohort_id)
                .expect("supported coindays-destroyed cohort");
            LazyPerBlockCumulativeRolling::from_cumulative_source(
                &name,
                version,
                source,
                cached_starts,
                mappings,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
