use bitview_cohort::{CohortContext, UTXOGroups};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{CachedWindowStartVec, LazyPerBlockCumulativeRolling};
use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::metrics::CumulativeUTXOSources;

#[derive(Traversable)]
pub struct SpentOutputCount<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroups<LazyPerBlockCumulativeRolling<StoredU64>>,
    #[traversable(hidden)]
    pub stored: CumulativeUTXOSources<StoredU64, M>,
}

impl SpentOutputCount {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        let stored = CumulativeUTXOSources::forced_import(
            cache,
            db,
            "spent_utxo_count_cumulative",
            version,
        )?;
        let cohorts = UTXOGroups::new(|cohort_id| {
            let name = CohortContext::Utxo.metric_name(cohort_id, "spent_utxo_count");
            LazyPerBlockCumulativeRolling::from_cumulative_source(
                &name,
                version,
                stored
                    .stored
                    .get(cohort_id)
                    .expect("spent-output cohort source"),
                cached_starts,
                mappings,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
